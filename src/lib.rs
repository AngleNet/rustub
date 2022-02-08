mod catalog;
mod common;
mod concurrency;
mod execution;
mod recovery;
mod storage;
#[cfg(test)]
mod tests;
mod types;
mod tiny_planner;

use common::error::*;

#[macro_use]
extern crate log;
extern crate flexi_logger;
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests_me {
    use crate::tests::test_setup_logger;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
