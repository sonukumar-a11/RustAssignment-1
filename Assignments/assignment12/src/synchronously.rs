use std::time::Duration;

use async_std::task;
use futures::executor::block_on;

pub async fn func_1() {
    for i in 1..10 {
        print!("f1 ");
        if i == 5 {
            task::sleep(Duration::from_secs(2)).await;
        }
    }
}

pub async fn func_2() {
    for _i in 1..10 {
        print!("f2 ");
    }
}

pub async fn equate() {
    let first = func_1();
    let second = func_2();

    futures::join!(first, second);
}