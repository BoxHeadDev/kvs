// Import necessary I/O traits and types from the standard library
use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write};

// Import a custom Result type defined elsewhere in the crate
use crate::Result;

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
