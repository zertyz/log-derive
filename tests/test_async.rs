#![cfg(feature = "async_test")]

mod test_logger;

use async_trait::async_trait;
use crate::test_logger::THREAD_LOGGER;
use log::Level;
use log_derive::{logfn, logfn_inputs};

#[logfn(INFO)]
async fn async_function(ok: bool) -> Result<&'static str, &'static str> {
    if ok {
        return Ok("async Ok");
    } else {
        return Err("async Err");
    }
}

#[async_trait]
trait MyAsync {
    async fn async_trait_function(&self, ok: bool) -> Result<&'static str, &'static str> ;
}
struct MyStruct;
#[async_trait]
impl MyAsync for MyStruct {
    #[logfn_inputs(INFO, skip(self))]
    //#[logfn(INFO)]
    async fn async_trait_function(&self, ok: bool) -> Result<&'static str, &'static str> {
        if ok {
            return Ok("async Ok");
        } else {
            return Err("async Err");
        }
    }
}
impl MyStruct {
    #[logfn_inputs(INFO, skip())]
    async fn i_dont_care() {}
}

#[test]
fn async_works() {
    test_logger::init();

    futures_executor::block_on(async {
        assert_eq!(async_function(true).await, Ok("async Ok"));
        THREAD_LOGGER.assert_last_log("async_function() => \"async Ok\"", Level::Info, 10);
        assert_eq!(async_function(false).await, Err("async Err"));
        THREAD_LOGGER.assert_last_log("async_function() => \"async Err\"", Level::Info, 10);
        assert!(THREAD_LOGGER.is_empty())
    })
}

#[test]
fn async_works2() {
    test_logger::init();
    let block = futures_executor::block_on;

    assert_eq!(block(async_function(true)), Ok("async Ok"));
    THREAD_LOGGER.assert_last_log("async_function() => \"async Ok\"", Level::Info, 10);
    assert_eq!(block(async_function(false)), Err("async Err"));
    THREAD_LOGGER.assert_last_log("async_function() => \"async Err\"", Level::Info, 10);
    assert!(THREAD_LOGGER.is_empty())
}

#[test]
fn async_trait_works() {
    test_logger::init();

    let instance = MyStruct;

    futures_executor::block_on(async {
        assert_eq!(instance.async_trait_function(true).await, Ok("async Ok"));
        THREAD_LOGGER.assert_last_log("async_trait_function(self: <skipped>,ok: true)", Level::Info, 26);
        assert_eq!(instance.async_trait_function(false).await, Err("async Err"));
        THREAD_LOGGER.assert_last_log("async_trait_function(self: <skipped>,ok: false)", Level::Info, 26);
        assert!(THREAD_LOGGER.is_empty())
    })
}