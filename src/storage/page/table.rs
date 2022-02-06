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
