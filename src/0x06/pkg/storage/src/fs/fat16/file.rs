//! File
//!
//! reference: <https://wiki.osdev.org/FAT#Directories_on_FAT12.2F16.2F32>

use super::*;

#[derive(Debug, Clone)]
pub struct File {
    /// The current offset in the file
    pub offset: usize,
    /// The current cluster of this file
    current_cluster: Cluster,
    /// DirEntry of this file
    entry: DirEntry,
    /// The file system handle that contains this file
    handle: Fat16Handle,
}

impl File {
    pub fn new(handle: Fat16Handle, entry: DirEntry) -> Self {
        let cluster = entry.cluster;
        Self {
            offset: 0,
            current_cluster: entry.cluster,
            entry,
            handle,
        }
    }

    pub fn length(&self) -> usize {
        self.entry.size as usize
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        // read file content from disk
        //      CAUTION: file length / buffer size / offset
        //
        //      - `self.offset` is the current offset in the file in bytes
        //      - use `self.handle` to read the blocks
        //      - use `self.entry` to get the file's cluster
        //      - use `self.handle.cluster_to_sector` to convert cluster to sector
        //      - update `self.offset` after reading
        //      - update `self.cluster` with FAT if necessary
        let length = self.length();

        if self.offset >= length {
            return Ok(0);
        }

        let total_blocks = (length + BLOCK_SIZE - 1) / BLOCK_SIZE;
        let mut current_block = self.offset / BLOCK_SIZE;
        let mut block = Block::default();
        let sector = self.handle.cluster2sector(&self.entry.cluster);

        let mut bytes_read = 0;

        while bytes_read < buf.len() && self.offset < length && current_block < total_blocks {
            current_block = self.offset / BLOCK_SIZE;
            let current_offset = self.offset % BLOCK_SIZE;
            self.handle
                .inner
                .read_block(sector + current_block, &mut block)?;

            let block_remain = BLOCK_SIZE - current_offset;
            let buf_remain = buf.len() - bytes_read;
            let file_remain = length - self.offset;
            let to_read = buf_remain.min(block_remain).min(file_remain);

            buf[bytes_read..bytes_read + to_read]
                .copy_from_slice(&block[current_offset..current_offset + to_read]);

            bytes_read += to_read;
            self.offset += to_read;
        }

        Ok(bytes_read)
    }
}

// NOTE: `Seek` trait is not required for this lab
impl Seek for File {
    fn seek(&mut self, _pos: SeekFrom) -> Result<usize> {
        unimplemented!()
    }
}

// NOTE: `Write` trait is not required for this lab
impl Write for File {
    fn write(&mut self, _buf: &[u8]) -> Result<usize> {
        unimplemented!()
    }

    fn flush(&mut self) -> Result<()> {
        unimplemented!()
    }
}
