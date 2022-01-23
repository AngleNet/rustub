use crate::common::config::PageId;

/// DiskManager takes care of the allocation and deallocation of pages within a database. It performs
/// the reading and writing of pages to and from disk, providing a logical file layer within the
/// context of a DBMS.
pub trait DiskManager {
    /// Creates a new disk manager that writes to the specified database file.
    fn new(file: String) -> Self;

    /// Shut down the disk manager and close all the file resources.
    fn shutdown(&mut self);

    /// Write a page to the database file
    fn write_page(&mut self, pid: PageId, data: &[u8]);

    fn read_page(&mut self, pid: PageId, data: &mut [u8]);

    fn write_log(&mut self, data: &[u8]);

    fn read_log(&mut self, data: &mut [u8], offset: u32);

    fn num_flushes(&self) -> u32;

    fn is_flushed(&self) -> bool;

    fn num_writes(&self) -> u32;

    fn set_flush_log_future(&mut self, func: dyn Fn());

    fn has_flush_log_future(&self) -> bool;
}

pub struct FileBasedDiskManager {}

pub struct InMemDiskManager {}
