use std::{sync::{Arc, Mutex}, thread::{self}};

use system::CouponBox;


mod system;

fn main() {
    println!("Start..");

    let coupon_box = Arc::new(CouponBox::new(0.1));
    let coupon_cnt = Arc::new(Mutex::new(0u64));

    let mut thread_list = Vec::with_capacity(10);
    
    for i in 0..100 {
        coupon_box.create_coupon(i);
    }

    for i in 0..9 {
        let coupon_box = Arc::clone(&coupon_box);
        let coupon_cnt = Arc::clone(&coupon_cnt);
        let handle = thread::spawn(move || {
            loop {
                let coupon =  coupon_box.pop_coupon();
                match coupon {
                    Some(_) => {
                        println!("Coupon Pop! : {:?}", coupon.unwrap());
                        let mut cnt = coupon_cnt.lock().unwrap();
                        *cnt += 1;
                    }
                    None => {
                        println!("Coupon Empty!");
                        break;
                    }
                }
            }
        });
        thread_list.push(handle);
    }


    for th in thread_list {
        println!("End..");
        th.join().unwrap();
    }

    println!("Popped Coupon cnt : {}", *(coupon_cnt.lock().unwrap()));
    
}