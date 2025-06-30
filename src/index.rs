use std::ops::Range;

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
