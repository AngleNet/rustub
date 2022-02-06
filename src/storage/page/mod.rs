use crate::common::config::{INVALID_PAGE_ID, PAGE_SIZE, PageId};

mod header;
mod table;
mod tmp;

pub use header::HeaderPage;
pub use table::TablePage;
pub use tmp::TmpTuplePage;

// Page
//  |__ HeaderPage
//  |__ TablePage
//  |__ TmpTuplePage

/// Page is the basic unit of storage within the database system. Page provides a wrapper for actual
/// data pages being held in main memory. Page also contains book-keeping information that is used by
/// the buffer pool manager. e.g. pin count, dirty flag, page id, etc.
pub trait Page {
    /// Returns the actual data contained within this page
    fn data(&self) -> &[u8];

    fn data_mut(&mut self) -> &mut [u8];

    /// Get the page id of this page
    fn page_id(&self) -> PageId;

    /// Returns true if the page in memory has been modified from the page on disk.
    fn is_dirty(&self) -> bool;

    /// Returns the pin count of this page
    fn pin_count(&self) -> usize;
}

struct BasePage {
    /// The actual data that is stored within the page
    data: [u8; PAGE_SIZE],
    /// The ID of this page
    page_id: PageId,
    /// The pin count of this page
    pin_count: usize,
    /// True if the page is dirty, i.e. it is different from its corresponding page on disk
    is_dirty: bool,
}

impl BasePage {
    pub fn new() -> Self {
        BasePage {
            data: [0; PAGE_SIZE],
            page_id: INVALID_PAGE_ID,
            pin_count: 0,
            is_dirty: false,
        }
    }
}

impl Page for BasePage {
    fn data(&self) -> &[u8] {
        &self.data[..]
    }

    fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data[..]
    }

    fn page_id(&self) -> PageId {
        self.page_id
    }

    fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    fn pin_count(&self) -> usize {
        self.pin_count
    }
}
