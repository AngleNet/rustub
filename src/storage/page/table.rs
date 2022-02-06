use bytes::{Buf, BufMut};
use crate::common::config::PageId;
use crate::storage::page::{BasePage, Page};

const SIZE_TABLE_PAGE_HEADER: usize = 24;
const SIZE_TUPLE: usize = 8;
const OFFSET_PREV_PAGE_ID: usize = 8;
const OFFSET_NEXT_PAGE_ID: usize = 12;
const OFFSET_FREE_SPACE: usize = 16;
const OFFSET_TUPLE_COUNT: usize = 20;
const OFFSET_TUPLE_OFFSET: usize = 24;
const OFFSET_TUPLE_SIZE: usize = 28;


/// Slotted page format:
/// ----------------------------------------------------------
/// |  header | ... free space ... | ... inserted tuples ... |
/// ----------------------------------------------------------
///                                ^
///                                free space pointer
/// Header format:
/// -------------------------------------------------------------------------------------------
/// | page id (4) | LSN (4) | previous page id (4) | next page id (4) | free space pointer (4)|
/// -------------------------------------------------------------------------------------------
///
/// -----------------------------------------------------------------
/// | tuple count (4) | tuple_1 offset (4) | tuple_1 size (4) | ... |
/// -----------------------------------------------------------------
pub struct TablePage {
    base: BasePage,
}

impl TablePage {
    /// Returns the page id of this table page
    pub fn page_id(&self) -> PageId {
        self.data().get_i32()
    }

    /// Returns the page id of the previous table page
    pub fn prev_page_id(&self) -> PageId {
        (&self.data()[OFFSET_PREV_PAGE_ID..]).get_i32()
    }

    /// Returns the page id of the next table page
    pub fn next_page_id(&self) -> PageId {
        (&self.data()[OFFSET_NEXT_PAGE_ID..]).get_i32()
    }

    /// Set the page id of the next table page
    pub fn set_next_page_id(&mut self, pid: PageId) {
        (&mut self.data_mut()[OFFSET_NEXT_PAGE_ID..]).put_i32(pid)
    }

    /// Set the page id of the previous table page
    pub fn set_prev_page_id(&mut self, pid: PageId) {
        (&mut self.data_mut()[OFFSET_PREV_PAGE_ID..]).put_i32(pid)
    }
}

impl Page for TablePage {
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
