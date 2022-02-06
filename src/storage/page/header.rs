use std::io::{Read, Write};
use std::mem::transmute;
use std::process::id;
use bytes::{Buf, BufMut};
use crate::common::config::{INVALID_PAGE_ID, PageId};
use crate::common::memcpy;
use crate::storage::page::{BasePage, Page};

const HEADER_PAGE_COUNT_SIZE: usize = 4;
const HEADER_PAGE_ENTRY_KEY_SIZE: usize = 32;
const HEADER_PAGE_ENTRY_VALUE_SIZE: usize = 4;
const HEADER_PAGE_ENTRY_SIZE: usize = HEADER_PAGE_ENTRY_KEY_SIZE + HEADER_PAGE_ENTRY_VALUE_SIZE;

/// Database use this first page (page_id = 0) as header page to store metadata, in our case, we will
/// contain information about table/index name (length less than 32 bytes) and their corresponding
/// root_id
///
/// Format (size in byte):
/// ---------------------------------------------------------
/// | record count (4) |   name (32)   | root id (4) | ... |
/// ---------------------------------------------------------
pub struct HeaderPage {
    base: BasePage,
}

impl HeaderPage {
    pub fn new() -> Self {
        todo!()
    }

    pub fn insert_record(&mut self, name: &str, root_id: PageId) -> bool {
        assert!(name.len() < HEADER_PAGE_ENTRY_KEY_SIZE);
        assert!(root_id > INVALID_PAGE_ID);

        if self.find(name) != -1 {
            // already exists
            return false;
        }
        let num = self.record_count() as usize;
        let offset = HEADER_PAGE_COUNT_SIZE + num * HEADER_PAGE_ENTRY_SIZE;
        let mut buf = &mut self.data_mut()[offset..];
        buf.put_slice(name.as_bytes());
        buf.put_i32(root_id);
        return true;
    }

    pub fn delete_record(&mut self, name: &str) -> bool {
        unimplemented!()
    }

    pub fn update_record(&mut self, name: &str, root_id: PageId) -> bool {
        assert!(name.len() < HEADER_PAGE_ENTRY_KEY_SIZE);
        let idx = self.find(name);
        if idx == -1 {
            return false;
        }
        let offset = idx as usize * HEADER_PAGE_ENTRY_SIZE + HEADER_PAGE_COUNT_SIZE + HEADER_PAGE_ENTRY_KEY_SIZE;
        (&mut (self.data_mut()[offset as usize..])).put_i32(root_id);
        return true;
    }

    pub fn root_id(&self, name: &str) -> PageId {
        assert!(name.len() < HEADER_PAGE_ENTRY_KEY_SIZE);

        let idx = self.find(name);
        if idx == -1 {
            return INVALID_PAGE_ID;
        }
        (&(self.data())[((idx + 1) * 36) as usize..]).get_i32()
    }

    pub fn record_count(&self) -> u32 {
        self.data().get_u32()
    }

    fn find(&self, name: &str) -> i32 {
        let count = self.record_count() as i32;
        for i in 0..count {
            if name.as_bytes().eq(&self.data()[(4 + i << 5) as usize..(36 + i << 5) as usize]) {
                return i;
            }
        }
        return -1;
    }

    fn set_record_count(&mut self, count: u32) {
        self.data_mut().put_u32(count)
    }
}

impl Page for HeaderPage {
    fn data(&self) -> &[u8] {
        self.base.data()
    }

    fn data_mut(&mut self) -> &mut [u8] {
        self.base.data_mut()
    }

    fn page_id(&self) -> PageId {
        self.base.page_id()
    }

    fn is_dirty(&self) -> bool {
        self.base.is_dirty()
    }

    fn pin_count(&self) -> usize {
        self.base.pin_count()
    }
}


#[cfg(test)]
mod tests {
    use bytes::Buf;

    #[test]
    fn test() {
        let x: [u8; 4] = [1, 2, 3, 4];
        let mut y = &x[..];
        let z = y.get_u8();
        println!("{}", y.len());
    }
}
