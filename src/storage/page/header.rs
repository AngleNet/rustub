use crate::common::config::{PageId, INVALID_PAGE_ID};
use crate::common::{memcpy, memmove};
use crate::storage::page::{BasePage, Page};
use bytes::{Buf, BufMut};
use std::io::{Read, Write};
use std::mem::transmute;
use std::process::id;

const HEADER_PAGE_COUNT_SIZE: usize = 4;
const HEADER_PAGE_ENTRY_KEY_SIZE: usize = 32;
const HEADER_PAGE_ENTRY_VALUE_SIZE: usize = 4;
const HEADER_PAGE_ENTRY_SIZE: usize = HEADER_PAGE_ENTRY_KEY_SIZE + HEADER_PAGE_ENTRY_VALUE_SIZE;

/// Database use this first page (page_id = 0) as header page to store metadata, in our case, we will
/// contain information about table/index name (length less than 32 bytes) and their corresponding
/// root page id.
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
        let mut page = HeaderPage {
            base: BasePage::new(),
        };
        page.set_record_count(0);
        return page;
    }

    pub fn insert_record(&mut self, name: &str, root_id: PageId) -> bool {
        assert!(name.len() < HEADER_PAGE_ENTRY_KEY_SIZE);
        assert!(root_id > INVALID_PAGE_ID);

        if self.find(name) != -1 {
            // already exists
            return false;
        }
        let num = self.record_count();
        let offset = HEADER_PAGE_COUNT_SIZE + num as usize * HEADER_PAGE_ENTRY_SIZE;
        (&mut self.data_mut()[offset..]).put_slice(name.as_bytes());
        (&mut self.data_mut()[offset + HEADER_PAGE_ENTRY_KEY_SIZE..]).put_i32(root_id);
        self.set_record_count(num + 1);
        return true;
    }

    pub fn delete_record(&mut self, name: &str) -> bool {
        assert!(name.len() < HEADER_PAGE_ENTRY_KEY_SIZE);
        let idx = self.find(name);
        if idx == -1 {
            return false;
        }
        let offset = HEADER_PAGE_COUNT_SIZE + idx as usize * HEADER_PAGE_ENTRY_SIZE;
        let count = self.record_count();
        // todo: fix this unsafe block
        unsafe {
            let start = self.data_mut().as_mut_ptr() as usize;
            let dest = start + offset * idx as usize;
            let src = dest + offset;
            let n = (count - idx as u32 - 1) as usize * HEADER_PAGE_ENTRY_SIZE;
            memmove(dest as *mut u8, src as *const u8, n);
        }
        self.set_record_count(self.record_count() - 1);
        true
    }

    pub fn update_record(&mut self, name: &str, root_id: PageId) -> bool {
        assert!(name.len() < HEADER_PAGE_ENTRY_KEY_SIZE);
        let idx = self.find(name);
        if idx == -1 {
            return false;
        }
        let offset = idx as usize * HEADER_PAGE_ENTRY_SIZE
            + HEADER_PAGE_COUNT_SIZE
            + HEADER_PAGE_ENTRY_KEY_SIZE;
        (&mut (self.data_mut()[offset as usize..])).put_i32(root_id);
        return true;
    }

    pub fn root_id(&self, name: &str) -> PageId {
        assert!(name.len() < HEADER_PAGE_ENTRY_KEY_SIZE);

        let idx = self.find(name);
        if idx == -1 {
            return INVALID_PAGE_ID;
        }
        (&self.data()[(idx + 1) as usize * HEADER_PAGE_ENTRY_SIZE..]).get_i32()
    }

    pub fn record_count(&self) -> u32 {
        self.data().get_u32()
    }

    fn find(&self, name: &str) -> i32 {
        let count = self.record_count();
        for i in 0..count {
            let offset = HEADER_PAGE_COUNT_SIZE + i as usize * HEADER_PAGE_ENTRY_SIZE;
            if name
                .as_bytes()
                .eq(&self.data()[offset..offset + name.len()])
            {
                return i as i32;
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
    use crate::common::config::INVALID_PAGE_ID;
    use crate::storage::page::HeaderPage;

    #[test]
    fn header_page_crud() {
        let mut page = HeaderPage::new();
        let key = "rustub";
        assert_eq!(page.record_count(), 0);
        assert!(page.insert_record(key, 1));
        assert_eq!(page.record_count(), 1);
        assert!(!page.insert_record(key, 1));
        assert!(page.update_record(key, 2));
        assert_eq!(page.root_id(key), 2);
        assert!(page.delete_record(key));
        assert_eq!(page.record_count(), 0);
        assert_eq!(page.root_id(key), INVALID_PAGE_ID);
    }
}
