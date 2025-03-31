use core::f32;
use std::sync::Arc;

use std::sync::Mutex;

pub trait Entity {
    fn get_id(&self) -> u64;
}

#[derive(Debug)]
pub struct Coupon {
    coupon_id: u64,
    category: u64,
    percent: f32
}

#[derive(Debug)]
pub struct CouponBox {
    list: Arc<Mutex<Vec<Coupon>>>,
    percent: f32
}

impl Entity for Coupon {
    fn get_id(&self) -> u64 {
        self.coupon_id
    }
}

impl Coupon {
    

    pub fn new(id:u64, category:u64, percent:f32) -> Self{
        Self {
            coupon_id: id,
            category,
            percent
        }
    }

    pub fn get_percent(&self) -> f32 {
        self.percent
    }

}

impl CouponBox {
    pub fn new(percent: f32) -> Self {
        Self {
            list: Arc::new(Mutex::new(Vec::new())),
            percent
        }
    }

    pub fn pop_coupon(&self) -> Option<Coupon> {

        let mut locked_list = self.list.lock().unwrap();
        let len = locked_list.len();
        
        if len > 0 {
            locked_list.pop()
        } else {
            None
        }
    }

    pub fn create_coupon(&self, id:u64) {
        let coupon = Coupon::new(id, 1, self.percent);
        println!("Create Coupon {:?}", coupon);
        let mut locked_list = self.list.lock().unwrap();
        locked_list.push(coupon);
        
    }

}