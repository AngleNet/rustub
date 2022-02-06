use std::fs;
use std::sync::Mutex;
use crate::common::config::{PAGE_SIZE, PageId};
use crate::common::error::Result;
use std::io::{Read, Seek, SeekFrom, Write};
use std::sync::atomic::AtomicBool;
use crate::RustubError;

pub type FlushLogFuture = fn();

/// DiskManager takes care of the allocation and deallocation of pages within a database. It performs
/// the reading and writing of pages to and from disk, providing a logical file layer within the
/// context of a DBMS.
pub trait DiskManager {
    /// Write a page to the database file
    fn write_page(&mut self, pid: PageId, data: &[u8]);

    fn read_page(&mut self, pid: PageId, data: &mut [u8]);

    fn write_log(&mut self, data: &[u8]);

    fn read_log(&mut self, data: &mut [u8], offset: u32) -> bool;

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
        // todo: How to initialize a database file safely?
        // Try to open the log file and truncate it if it already exists.
        let mut file = fs::File::options().create(true).append(true).read(true).open(&log_file);
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
        file = fs::File::options().create(true).write(true).read(true).open(&db_file);
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
        // todo: this does not fail in bustub
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
        assert_eq!(data.len(), PAGE_SIZE);
        let offset = pid as usize * PAGE_SIZE;
        self.db_io.seek(SeekFrom::Start(offset as u64)).unwrap();
        if let Err(e) = self.db_io.write_all(data) {
            error!("IO error while writing page: {}", e);
        }
        self.db_io.flush();
    }

    /// Read the contents of the specified page into the given buf.
    ///
    /// Reminders:
    ///
    /// THREAD SAFETY: NO
    fn read_page(&mut self, pid: PageId, mut data: &mut [u8]) {
        assert_eq!(data.len(), PAGE_SIZE);
        let offset = pid as usize * PAGE_SIZE;
        if offset > FileBasedDiskManager::get_file_size(&self.db_file) {
            error!("IO error while reading past end of file");
        } else {
            self.db_io.seek(SeekFrom::Start(offset as u64));
            // todo: why could we ignore the failure of seek?
            while !data.is_empty() {
                match self.db_io.read(data) {
                    // the read has reached its 'end-of-file'
                    Ok(0) => break,
                    Ok(n) => {
                        let tmp = data;
                        data = &mut tmp[n..];
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                    Err(e) => {
                        error!("IO error while reading page");
                        return;
                    }
                }
            }
            if !data.is_empty() {
                debug!("Read less than a page");
                data.fill(0u8);
            }
        }
    }

    /// Write the contents of the log into disk file. Only return when sync is done, and only perform
    /// sequential write.
    ///
    /// THREAD-SAFETY: NO
    fn write_log(&mut self, data: &[u8]) {
        if data.is_empty() {
            return;
        }
        // todo: try to make this async
        self.num_flushes += 1;
        if self.log_io.write_all(data).is_err() {
            error!("IO error while writing log");
            return;
        }
        self.log_io.flush().unwrap();
    }

    /// Read the contents of the log into the given buf. Always read from the beginning and perform
    /// sequential read.
    ///
    /// Returns false means already reach the end.
    ///
    /// THREAD SAFETY: NO
    fn read_log(&mut self, mut data: &mut [u8], offset: u32) -> bool {
        if offset >= FileBasedDiskManager::get_file_size(&self.log_file) as u32 {
            debug!("end of log file");
            return false;
        }
        self.log_io.seek(SeekFrom::Start(offset as u64));
        while !data.is_empty() {
            match self.log_io.read(data) {
                // the read has reached its 'end-of-file'
                Ok(0) => {
                    break;
                }
                Ok(n) => {
                    let tmp = data;
                    data = &mut tmp[n..];
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                Err(e) => {
                    error!("IO error while reading page");
                    return false;
                }
            }
        }
        if !data.is_empty() {
            debug!("Read less than a page");
            data.fill(0u8);
        }
        return true;
    }

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
    use std::sync::Once;
    use flexi_logger::{colored_default_format, colored_opt_format};

    static TEST_LOGGER_INIT: Once = Once::new();

    fn test_setup_logger() {
        TEST_LOGGER_INIT.call_once(|| {
            flexi_logger::Logger::try_with_env_or_str("debug").unwrap()
                .log_to_stdout()
                .format_for_stdout(colored_opt_format)
                .use_utc()
                .start()
                .expect("the logger should start");
        });
    }

    use std::fs;
    use std::fs::File;
    use std::io::{Read, Seek, SeekFrom, Write};
    use std::panic;
    use crate::common::config::PAGE_SIZE;
    use crate::common::memcpy;
    use crate::storage::disk::{DiskManager, FileBasedDiskManager};

    fn set_up() {
        fs::remove_file("test.db");
        fs::remove_file("test.log");
        test_setup_logger();
    }

    fn tear_down() {
        fs::remove_file("test.db");
        fs::remove_file("test.log");
    }

    // todo: how to setup and teardown tests in rust?
    fn run_test<T>(test: T) -> ()
        where T: FnOnce() -> () + panic::UnwindSafe
    {
        set_up();

        let result = panic::catch_unwind(|| {
            test()
        });

        tear_down();
        assert!(result.is_ok())
    }

    #[test]
    fn test_read_write_page() {
        run_test(|| {
            let mut buf = [0u8; PAGE_SIZE];
            let mut data = [0u8; PAGE_SIZE];
            let db_file = "test.db".to_string();
            let mut dm = FileBasedDiskManager::new(db_file).unwrap();
            let test_str = &b"A test string."[..];
            // todo: refactor this
            unsafe {
                memcpy(data.as_mut_ptr(), test_str.as_ptr(), test_str.len());
            }

            // tolerate empty read
            dm.read_page(0, &mut buf[..]);

            assert_eq!(buf, [0u8; PAGE_SIZE]);

            dm.write_page(0, &data[..]);
            dm.read_page(0, &mut buf[..]);
            assert_eq!(buf, data);

            buf.fill(0);
            dm.write_page(5, &mut data[..]);
            dm.read_page(0, &mut buf[..]);
            assert_eq!(buf, data);
        })
    }

    #[test]
    fn test_read_write_log() {
        let mut buf = [0u8; 16];
        let mut data = [0u8; 16];
        let db_file = "test.db".to_string();
        let mut dm = FileBasedDiskManager::new(db_file).unwrap();

        let test_str = &b"A test string."[..];
        // todo: refactor this
        unsafe {
            memcpy(data.as_mut_ptr(), test_str.as_ptr(), test_str.len());
        }

        // dm.read_log(&mut buf[..], 0);

        dm.write_log(&data[..]);
        dm.read_log(&mut buf[..], 0);
        assert_eq!(buf, data);
    }

    #[test]
    fn test_append_read_write() {
        let mut log = File::options().append(true).create(true).read(true)
            .open("test.log").unwrap();

        log.write(&b"12345"[..]).unwrap();
        log.flush();
        let mut buf = [0u8; 1];

        // Read will change the file cursor. If a write happens after the read, the read cursor will
        // be reset to the last seek.
        log.seek(SeekFrom::Start(1));
        log.read(&mut buf[..]).unwrap();
        assert_eq!(buf[0], '2' as u8);

        log.read(&mut buf[..]).unwrap();
        assert_eq!(buf[0], '3' as u8);

        log.read(&mut buf[..]).unwrap();
        assert_eq!(buf[0], '4' as u8);

        log.write(&b"678"[..]).unwrap();
        log.flush();

        log.read(&mut buf[..]).unwrap();
        assert_eq!(buf[0], '4' as u8);

        log.read(&mut buf[..]).unwrap();
        assert_eq!(buf[0], '4' as u8);

        log.read(&mut buf[..]).unwrap();
        assert_eq!(buf[0], '4' as u8);

        log.write(&b"90"[..]).unwrap();
        log.flush();
        log.read(&mut buf[..]).unwrap();
        assert_eq!(buf[0], '4' as u8);
    }
}
