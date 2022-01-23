use std::fs::File;
use std::sync::Mutex;
use crate::common::config::PageId;
use std::io::Result;

pub type FlushLogFuture = fn();

/// DiskManager takes care of the allocation and deallocation of pages within a database. It performs
/// the reading and writing of pages to and from disk, providing a logical file layer within the
/// context of a DBMS.
pub trait DiskManager {
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

    fn set_flush_log_future(&mut self, func: FlushLogFuture);

    fn has_flush_log_future(&self) -> bool;
}

pub struct FileBasedDiskManager {
    db_file: String,
    log_file: String,
    db_io: File,
    log_io: File,

    num_flushes: u32,
    num_writes: u32,
    flush_log: bool,
    flush_log_func: Option<FlushLogFuture>,
}

impl FileBasedDiskManager {
    fn new(db_file: String) -> Result<FileBasedDiskManager> {
        // fixme: check file location
        let db_io = File::options().create(true).read(true).write(true).open(&db_file)?;
        let log_io = File::options().create(true).write(true).append(true).open(&db_file)?;
        Ok(FileBasedDiskManager {
            db_file,
            log_file: "".to_string(),
            db_io,
            log_io,
            num_flushes: 0,
            num_writes: 0,
            flush_log: false,
            flush_log_func: None,
        })
    }

    fn get_file_size(file_name: &String) -> usize {
        todo!()
    }
}


impl DiskManager for FileBasedDiskManager {
    fn shutdown(&mut self) {
        todo!()
    }

    fn write_page(&mut self, pid: PageId, data: &[u8]) {
        todo!()
    }

    fn read_page(&mut self, pid: PageId, data: &mut [u8]) {
        todo!()
    }

    fn write_log(&mut self, data: &[u8]) {
        todo!()
    }

    fn read_log(&mut self, data: &mut [u8], offset: u32) {
        todo!()
    }

    fn num_flushes(&self) -> u32 {
        todo!()
    }

    fn is_flushed(&self) -> bool {
        todo!()
    }

    fn num_writes(&self) -> u32 {
        todo!()
    }

    fn set_flush_log_future(&mut self, func: FlushLogFuture) {
        todo!()
    }

    fn has_flush_log_future(&self) -> bool {
        todo!()
    }
}

pub struct InMemDiskManager {}
