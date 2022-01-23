mod types;
mod catalog;
mod storage;
mod concurrency;
mod execution;
mod recovery;
mod common;
#[cfg(test)]
mod tests;

use common::error::*;

#[macro_use]
extern crate log;
extern crate flexi_logger;


#[cfg(test)]
mod tests_me {
    use crate::tests::test_setup_logger;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
