use std::time::Duration;

use async_std::task;
use log::*;

/// check_table which used to async function that polls two futures simultaneously to print two tables in asynchronous manner.
///
/// #Arguments
///
/// No Arguments.
///
/// #Return
///
/// The returned future will finish with a tuple of both results.
pub async fn check_table() {
    let two_table = async {
        for index in 0..11 {
            info!("2*{} = {} ", index, 2 * index);
            task::sleep(Duration::from_secs(2)).await;
        }
    };
    let third_table = async {
        for index in 0..11 {
            debug!("3*{} = {} ", index, 3 * index);
            task::sleep(Duration::from_secs(1)).await;
        }
    };
    futures::join!(two_table, third_table);
}
