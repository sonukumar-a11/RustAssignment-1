use std::time::Duration;

use async_std::task;
use log::*;

/// check_process 1 that transforms a block of code into a state machine  through Future
///
/// #Arguments
///
/// No Arguments.
///
/// #Return
///
/// return the impl<future>
pub async fn check_process1() {
    for i in 1..10 {
        info!("Process1 ");
        if i == 9 {
            task::sleep(Duration::from_secs(2)).await;
        }
    }
}

/// check_process2 waits for the future to complete.
///
/// #Arguments
///
/// No Arguments.
///
/// #Return
///
/// return the impl<future>
pub async fn check_process2() {
    for _i in 1..10 {
        info!("process2 ");
    }
}

/// equate 1 which used to async function that
///
/// #Arguments
///
/// No Arguments.
///
/// #Return
///
/// The returned future will finish with a tuple of both results.
pub async fn equate() {
    let first = check_process1();
    let second = check_process2();

    futures::join!(first, second);
}
