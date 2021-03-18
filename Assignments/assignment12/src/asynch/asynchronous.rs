use std::time::Duration;

use async_std::task;
use log::*;

pub async fn compute() {
    env_logger::init();
    let first = async {
        for index in 1..11 {
            debug!("2*{} = {} ", index, 2 * index);
            task::sleep(Duration::from_secs(1)).await;
        }
    };
    let second = async {
        for index in 1..11 {
            debug!("3*{} = {} ", index, 3 * index);
            task::sleep(Duration::from_secs(1)).await;
        }
    };
    futures::join!(first, second);
}
