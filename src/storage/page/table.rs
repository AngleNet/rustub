use crate::common::config::PageId;
use crate::storage::page::{BasePage, Page};

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
