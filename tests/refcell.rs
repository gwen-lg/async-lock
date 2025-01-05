use async_lock::RefCell;
use futures_lite::future;
use std::{rc::Rc, thread::sleep, time::Duration};

#[test]
fn test_refcell() {
    future::block_on(async {
        let counter_refcell = Rc::new(RefCell::new(0));
        let counter = counter_refcell.borrow().await;
        println!("counter value : {}", counter);
        sleep(Duration::from_secs(10));
        //let counter_mut = counter_refcell.borrow_mut().await;
    });
}
