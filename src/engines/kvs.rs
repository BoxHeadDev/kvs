use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use std::collections::{BTreeMap, HashMap};
use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use super::KvsEngine;
use crate::{KvsError, Result};

const COMPACTION_THRESHOLD: u64 = 1024 * 1024;

/// The `KvStore` stores string key/value pairs.
///
/// Key/value pairs are persisted to disk in log files. Log files are named after
/// monotonically increasing generation numbers with a `log` extension name.
/// A `BTreeMap` in memory stores the keys and the value locations for fast query.
///
/// ```rust
/// # use kvs::{KvsEngine, KvStore, Result};
/// # fn try_main() -> Result<()> {
/// use std::env::current_dir;
/// let mut store = KvStore::open(current_dir()?)?;
/// store.set("key".to_owned(), "value".to_owned())?;
/// let val = store.get("key".to_owned())?;
/// assert_eq!(val, Some("value".to_owned()));
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct KvStore {
    imp: Arc<Mutex<KvStoreImpl>>,
}

struct KvStoreImpl {
    // directory for the log and other data.
    path: PathBuf,
    // map generation number to the file reader.
    readers: HashMap<u64, BufReaderWithPos<File>>,
    // writer of the current log.
    writer: BufWriterWithPos<File>,
    current_file: u64,
    index: BTreeMap<String, CommandPos>,
    // the number of bytes representing "stale" commands that could be
    // deleted during a compaction.
    uncompacted: u64,
}

impl KvsEngine for KvStore {
    /// Opens a `KvStore` with the given path.
    ///
    /// This will create a new directory if the given one does not exist.
    ///
    /// # Errors
    ///
    /// It propagates I/O or deserialization errors during the log replay.
    fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path.into();
        fs::create_dir_all(&path)?;

        let mut readers = HashMap::new();
        let mut index = BTreeMap::new();

        let file_list = sorted_file_list(&path)?;
        let mut uncompacted = 0;

        for &file_id in &file_list {
            let mut reader = BufReaderWithPos::new(File::open(log_path(&path, file_id))?)?;
            uncompacted += load(file_id, &mut reader, &mut index)?;
            readers.insert(file_id, reader);
        }

        let current_file = file_list.last().unwrap_or(&0) + 1;
        let writer = new_log_file(&path, current_file, &mut readers)?;

        let imp = KvStoreImpl {
            path,
            readers,
            writer,
            current_file,
            index,
            uncompacted,
        };

        Ok(KvStore {
            imp: Arc::new(Mutex::new(imp)),
        })
    }

    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    ///
    /// # Errors
    ///
    /// It propagates I/O or serialization errors during writing the log.
    fn set(&self, key: String, value: String) -> Result<()> {
        self.imp.lock().unwrap().set(key, value)
    }

    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    fn get(&self, key: String) -> Result<Option<String>> {
        self.imp.lock().unwrap().get(key)
    }

    /// Removes a given key.
    ///
    /// # Error
    ///
    /// It returns `KvsError::KeyNotFound` if the given key is not found.
    ///
    /// It propagates I/O or serialization errors during writing the log.
    fn remove(&self, key: String) -> Result<()> {
        self.imp.lock().unwrap().remove(key)
    }
}

impl KvStoreImpl {
    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    ///
    /// # Errors
    ///
    /// It propagates I/O or serialization errors during writing the log.
    fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::set(key, value);
        let pos = self.writer.pos;

        serde_json::to_writer(&mut self.writer, &cmd)?;
        self.writer.flush()?;

        if let Command::Set { key, .. } = cmd {
            if let Some(old_cmd) = self
                .index
                .insert(key, (self.current_file, pos..self.writer.pos).into())
            {
                self.uncompacted += old_cmd.len;
            }
        }
        if self.uncompacted > COMPACTION_THRESHOLD {
            self.compact()?;
        }
        Ok(())
    }

    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    ///
    /// # Errors
    ///
    /// It returns `KvsError::UnexpectedCommandType` if the given command type unexpected.
    fn get(&mut self, key: String) -> Result<Option<String>> {
        if let Some(cmd_pos) = self.index.get(&key) {
            let reader = self
                .readers
                .get_mut(&cmd_pos.file_id)
                .expect("Cannot find log reader");
            reader.seek(SeekFrom::Start(cmd_pos.pos))?;
            let cmd_reader = reader.take(cmd_pos.len);
            if let Command::Set { value, .. } = serde_json::from_reader(cmd_reader)? {
                Ok(Some(value))
            } else {
                Err(KvsError::UnexpectedCommandType)
            }
        } else {
            Ok(None)
        }
    }

    /// Removes a given key.
    ///
    /// # Errors
    ///
    /// It returns `KvsError::KeyNotFound` if the given key is not found.
    ///
    /// It propagates I/O or serialization errors during writing the log.
    fn remove(&mut self, key: String) -> Result<()> {
        if self.index.contains_key(&key) {
            let cmd = Command::remove(key);
            serde_json::to_writer(&mut self.writer, &cmd)?;
            self.writer.flush()?;
            if let Command::Remove { key } = cmd {
                let old_cmd = self.index.remove(&key).expect("key not found");
                self.uncompacted += old_cmd.len;
            }
            Ok(())
        } else {
            Err(KvsError::KeyNotFound)
        }
    }

    /// Clears stale entries in the log.
    pub fn compact(&mut self) -> Result<()> {
        // increase current gen by 2. current_gen + 1 is for the compaction file.
        let compaction_file = self.current_file + 1;
        self.current_file += 2;
        self.writer = self.new_log_file(self.current_file)?;

        let mut compaction_writer = self.new_log_file(compaction_file)?;

        let mut new_pos = 0; // pos in the new log file.
        for cmd_pos in &mut self.index.values_mut() {
            let reader = self
                .readers
                .get_mut(&cmd_pos.file_id)
                .expect("Cannot find log reader");
            if reader.pos != cmd_pos.pos {
                reader.seek(SeekFrom::Start(cmd_pos.pos))?;
            }

            let mut entry_reader = reader.take(cmd_pos.len);
            let len = io::copy(&mut entry_reader, &mut compaction_writer)?;
            *cmd_pos = (compaction_file, new_pos..new_pos + len).into();
            new_pos += len;
        }
        compaction_writer.flush()?;

        // remove stale log files.
        let stale_files: Vec<_> = self
            .readers
            .keys()
            .filter(|&&file_id| file_id < compaction_file)
            .cloned()
            .collect();
        for stale_file in stale_files {
            self.readers.remove(&stale_file);
            fs::remove_file(log_path(&self.path, stale_file))?;
        }
        self.uncompacted = 0;

        Ok(())
    }

    /// Create a new log file with given generation number and add the reader to the readers map.
    ///
    /// Returns the writer to the log.
    fn new_log_file(&mut self, file_id: u64) -> Result<BufWriterWithPos<File>> {
        new_log_file(&self.path, file_id, &mut self.readers)
    }
}

fn load(
    file_id: u64,
    reader: &mut BufReaderWithPos<File>,
    index: &mut BTreeMap<String, CommandPos>,
) -> Result<u64> {
    // To make sure we read from the beginning of the file.
    let mut pos = reader.seek(SeekFrom::Start(0))?;
    let mut stream = Deserializer::from_reader(reader).into_iter::<Command>();
    let mut uncompacted = 0;
    while let Some(cmd) = stream.next() {
        let new_pos = stream.byte_offset() as u64;
        match cmd? {
            Command::Set { key, .. } => {
                if let Some(old_cmd) = index.insert(key, (file_id, pos..new_pos).into()) {
                    uncompacted += old_cmd.len;
                }
            }
            Command::Remove { key } => {
                if let Some(old_cmd) = index.remove(&key) {
                    uncompacted += old_cmd.len;
                }
                // the "remove" command itself can be deleted in the next compaction.
                // so we add its length to `uncompacted`.
                uncompacted += new_pos - pos;
            }
        }
        pos = new_pos;
    }
    Ok(uncompacted)
}

fn log_path(dir: &Path, file_id: u64) -> PathBuf {
    dir.join(format!("{}.log", file_id))
}

/// Returns sorted generation numbers in the given directory.
fn sorted_file_list(path: &Path) -> Result<Vec<u64>> {
    let mut file_list: Vec<u64> = fs::read_dir(path)?
        .flat_map(|res| -> Result<_> { Ok(res?.path()) })
        .filter(|path| path.is_file() && path.extension() == Some("log".as_ref()))
        .flat_map(|path| {
            path.file_name()
                .and_then(OsStr::to_str)
                .map(|s| s.trim_end_matches(".log"))
                .map(str::parse::<u64>)
        })
        .flatten()
        .collect();
    file_list.sort_unstable();
    Ok(file_list)
}

/// Create a new log file with given generation number and add the reader to the readers map.
///
/// Returns the writer to the log.
fn new_log_file(
    path: &Path,
    file_id: u64,
    readers: &mut HashMap<u64, BufReaderWithPos<File>>,
) -> Result<BufWriterWithPos<File>> {
    let path = log_path(path, file_id);
    let writer = BufWriterWithPos::new(OpenOptions::new().create(true).append(true).open(&path)?)?;
    readers.insert(file_id, BufReaderWithPos::new(File::open(&path)?)?);
    Ok(writer)
}

// Define a struct that wraps a BufReader and keeps track of the current read position
/// A buffered reader that tracks the current byte position in the stream.
///
/// This is useful for scenarios where you need to know how far into the input stream
/// you've read, such as parsing log files or implementing custom file formats.
pub struct BufReaderWithPos<R: Read + Seek> {
    /// The underlying buffered reader.
    pub reader: BufReader<R>, // The actual buffered reader
    /// The current byte offset from the start of the stream.
    pub pos: u64, // The current byte position in the stream
}

// Implement methods for BufReaderWithPos
impl<R: Read + Seek> BufReaderWithPos<R> {
    // Constructor that initializes the reader and position
    /// Creates a new `BufReaderWithPos` from a readable and seekable source.
    pub fn new(mut inner: R) -> Result<Self> {
        // Get the current position in the stream using SeekFrom::Current(0)
        let pos = inner.seek(SeekFrom::Current(0))?;
        // Return the struct, wrapping `inner` in a BufReader and storing the position
        Ok(BufReaderWithPos {
            reader: BufReader::new(inner),
            pos,
        })
    }
}

// Implement the Read trait so BufReaderWithPos can be used as a reader
impl<R: Read + Seek> Read for BufReaderWithPos<R> {
    /// Reads bytes into the buffer and updates the position accordingly.
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // Read bytes into the buffer
        let len = self.reader.read(buf)?;
        // Update the current position
        self.pos += len as u64;
        // Return the number of bytes read
        Ok(len)
    }
}

// Implement the Seek trait so we can move around in the stream
impl<R: Read + Seek> Seek for BufReaderWithPos<R> {
    /// Seeks to a new position in the stream and updates the internal position.
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        // Delegate the seek to the inner BufReader
        self.pos = self.reader.seek(pos)?;
        // Return the new position
        Ok(self.pos)
    }
}

// Define a struct that wraps a BufWriter and keeps track of the current write position
/// A buffered writer that tracks the current byte position in the stream.
///
/// This is useful for maintaining awareness of write offsets during log file creation
/// or structured data serialization.
pub struct BufWriterWithPos<W: Write + Seek> {
    /// The underlying buffered writer.
    pub writer: BufWriter<W>, // The actual buffered writer
    /// The current byte offset from the start of the stream.
    pub pos: u64, // The current byte position in the stream
}

// Implement methods for BufWriterWithPos
impl<W: Write + Seek> BufWriterWithPos<W> {
    // Constructor that initializes the writer and position
    /// Creates a new `BufWriterWithPos` from a writable and seekable source
    pub fn new(mut inner: W) -> Result<Self> {
        // Get the current position in the stream
        let pos = inner.seek(SeekFrom::Current(0))?;
        // Return the struct, wrapping `inner` in a BufWriter and storing the position
        Ok(BufWriterWithPos {
            writer: BufWriter::new(inner),
            pos,
        })
    }
}

// Implement the Write trait so BufWriterWithPos can be used to write data
impl<W: Write + Seek> Write for BufWriterWithPos<W> {
    /// Writes bytes from the buffer and updates the internal position.
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // Write bytes from the buffer
        let len = self.writer.write(buf)?;
        // Update the current position
        self.pos += len as u64;
        // Return the number of bytes written
        Ok(len)
    }

    // Flush buffered data to the underlying writer
    /// Flushes any buffered data to the underlying writer.
    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

// Implement the Seek trait so we can move around in the output stream
impl<W: Write + Seek> Seek for BufWriterWithPos<W> {
    /// Seeks to a new position in the stream and updates the internal position.
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        // Delegate the seek to the inner BufWriter
        self.pos = self.writer.seek(pos)?;
        // Return the new position
        Ok(self.pos)
    }
}

/// Represents the position of a command in a log file.
///
/// `CommandPos` stores metadata about where a command is located,
/// including which file it's in, where it starts (`pos`), and its length (`len`).
#[derive(Debug)]
pub struct CommandPos {
    /// The file ID or file number where the command is stored.
    pub file_id: u64,
    /// The starting byte offset of the command within the file.
    pub pos: u64,
    /// The length in bytes of the command.
    pub len: u64,
}

impl From<(u64, Range<u64>)> for CommandPos {
    /// Creates a `CommandPos` from a tuple containing a file ID and a byte range.
    fn from((file_id, range): (u64, Range<u64>)) -> Self {
        CommandPos {
            file_id,
            pos: range.start,
            len: range.end - range.start,
        }
    }
}

/// Represents a command in the key-value store.
///
/// The `Command` enum defines the supported operations:
/// - `Set`: Stores a value for a given key.
/// - `Remove`: Deletes the key and its associated value from the store.
#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    /// Set a value for the given key.
    ///
    /// # Fields
    /// - `key`: The key to associate with the value.
    /// - `value`: The value to store.
    Set {
        /// The key to associate with the value.
        key: String,
        /// The value to store.
        value: String,
    },

    /// Remove the value associated with the given key.
    ///
    /// # Fields
    /// - `key`: The key to remove from the store.
    Remove {
        /// The key to remove from the store.
        key: String,
    },
}

impl Command {
    fn set(key: String, value: String) -> Command {
        Command::Set { key, value }
    }

    fn remove(key: String) -> Command {
        Command::Remove { key }
    }
}
