use std::fs;
use std::sync::Mutex;
use crate::common::config::PageId;
use crate::common::error::Result;
use std::io::{Read, Seek, SeekFrom, Write};
use crate::RustubError;

pub const PageSize: usize = 4096;

pub type FlushLogFuture = fn();

/// DiskManager takes care of the allocation and deallocation of pages within a database. It performs
/// the reading and writing of pages to and from disk, providing a logical file layer within the
/// context of a DBMS.
pub trait DiskManager {
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
    db_io: fs::File,
    log_io: fs::File,

    num_flushes: u32,
    num_writes: u32,
    flush_log: bool,
    flush_log_func: Option<FlushLogFuture>,
}

impl FileBasedDiskManager {
    fn new(db_file: String) -> Result<FileBasedDiskManager> {
        let mut log_file = if let Some(idx) = db_file.rfind(".") {
            db_file[0..idx].to_string()
        } else {
            db_file.clone()
        };
        log_file.push_str(".log");
        // todo: refactor this.
        // Try to open the log file and truncate it if it already exists.
        let mut file = fs::File::options().append(true).read(true).open(&log_file);
        if file.is_err() {
            // the just opened file will be dropped right here
            file = fs::File::options().truncate(true).write(true).open(&log_file);
            // reopen it with the original options
            file = fs::File::options().append(true).read(true).open(&log_file);
            if file.is_err() {
                return Err(RustubError::IOError(file.err().unwrap(), "can't open log file"));
            }
        }
        let log_io = file.unwrap();
        file = fs::File::options().write(true).read(true).open(&db_file);
        if file.is_err() {
            file = fs::File::options().truncate(true).write(true).open(&db_file);
            file = fs::File::options().write(true).read(true).open(&db_file);
            if file.is_err() {
                return Err(RustubError::IOError(file.err().unwrap(), "can't open database file"));
            }
        }
        return Ok(FileBasedDiskManager {
            db_file,
            log_file,
            db_io: file.unwrap(),
            log_io,
            num_flushes: 0,
            num_writes: 0,
            flush_log: false,
            flush_log_func: None,
        });
    }

    fn get_file_size(file_name: &String) -> usize {
        let meta = fs::metadata(file_name).unwrap();
        return meta.len() as usize;
    }
}


impl DiskManager for FileBasedDiskManager {


    // 1. What does the APPEND flag do when open a file? For a write, we need to ensure the data is
    // passed to write once and the offset will be set to the end of file.
    // 2. How does the seek affect read and write?
    // 3. What if we try to seek to a position which is out of the size of file?

    /// Write the contents of the specified page into disk file. The page is flushed immediately.
    ///
    /// Reminders:
    ///     1. Is the write atomic ?
    ///     2. What if we try write to an offset that is beyond the file size? The size of the file
    ///     will be expanded and the gap will be filled with zero.
    ///
    /// THREAD SAFETY: NO
    fn write_page(&mut self, pid: PageId, data: &[u8]) {
        assert_eq!(data.len(), PageSize);
        let offset = pid as usize * PageSize;
        self.db_io.seek(SeekFrom::Start(offset as u64)).unwrap();
        if let Err(e) = self.db_io.write_all(data) {
            error!("IO error while writing page: {}", e);
        }
        self.db_io.flush();
    }

    fn read_page(&mut self, pid: PageId, data: &mut [u8]) {
        assert!(data.len() > 0 && data.len() <= PageSize as usize);
        let offset = pid as usize * PageSize;
        self.db_io.seek(SeekFrom::Start(offset as u64)).unwrap();
        if let Err(e) = self.db_io.read_exact(data) {
            debug!("I/O error while reading page");
        }
    }

    fn write_log(&mut self, data: &[u8]) {
        if let Err(e) = self.log_io.write_all(data) {
            debug!("I/O error while writing log");
        }
    }

    fn read_log(&mut self, data: &mut [u8], offset: u32) {}

    #[inline]
    fn num_flushes(&self) -> u32 {
        self.num_flushes
    }

    #[inline]
    fn is_flushed(&self) -> bool {
        self.flush_log
    }

    #[inline]
    fn num_writes(&self) -> u32 {
        self.num_writes
    }

    #[inline]
    fn set_flush_log_future(&mut self, func: FlushLogFuture) {
        self.flush_log_func = Some(func);
    }

    #[inline]
    fn has_flush_log_future(&self) -> bool {
        return self.flush_log_func.is_some();
    }
}

pub struct InMemDiskManager {}


#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::{Read, Seek, SeekFrom, Write};

    #[test]
    fn test() {
        let mut f1 = File::options().create(true).read(true).write(true).open("/tmp/test").unwrap();
        let one = "1".as_bytes();
        let two = "2".as_bytes();
        f1.seek(SeekFrom::Start(10)).unwrap();
        let mut x = [8; 10];
        f1.write(one);
        f1.flush();

        f1.seek(SeekFrom::Start(0)).unwrap();
        f1.read(&mut x[..]).unwrap();
        println!("{:?}", x);
    }

    #[test]
    fn test_a() {
        let mut a = A { a: 1 };
        {
            a = A { a: 2 };
        }
        a = A { a: 3 };
    }

    struct A {
        a: i32,
    }

    impl Drop for A {
        fn drop(&mut self) {
            println!("drop {}", self.a);
        }
    }
}
