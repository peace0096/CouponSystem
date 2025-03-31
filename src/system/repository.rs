
use std::{cell::OnceCell, sync::{Arc, Mutex, OnceLock}, vec};

use crate::system::object::*;

pub trait SingletonRepository<T> {
    fn instance() -> &'static Self where Self: Sized;
}

pub trait Repository<T> {
    fn save(&self, entity: T) -> bool;
    fn delete(&self, id: u64);
    fn get_count(&self) -> usize;
}

pub struct MemoryRepository<T> {
    list: Arc<Mutex<Vec<T>>>
}

macro_rules! impl_singleton_repository {
    ($t:ty) => {
        impl SingletonRepository<$t> for MemoryRepository<$t> {
            fn instance() -> &'static Self {
                static INSTANCE: OnceLock<MemoryRepository<$t>> = OnceLock::new();
                INSTANCE.get_or_init(|| {
                    MemoryRepository {
                        list: Arc::new(Mutex::new(Vec::new()))
                    }
                })
            }
        }
    };
}

impl<T> Repository<T> for MemoryRepository<T> {
    fn save(&self, entity: T) -> bool {
        self.list.lock().unwrap().push(entity);
        true
    }

    fn delete(&self, id: u64) {
        
    }
    
    fn get_count(&self) -> usize {
        self.list.lock().unwrap().len()
    }
}


pub trait CouponRepository {
    fn pop_coupon(&self) -> Option<Coupon>;
    fn create_coupon(&self, id:u64, category:u64) -> bool; 
}

impl CouponRepository for MemoryRepository<Coupon> {
    fn pop_coupon(&self) -> Option<Coupon> {
        let mut locked_list: std::sync::MutexGuard<'_, Vec<Coupon>> = self.list.lock().unwrap();
        let len = locked_list.len();
        
        if len > 0 {
            locked_list.pop()
        } else {
            None
        }
    }

    fn create_coupon(&self, id:u64, category:u64) -> bool {
        let coupon = Coupon::new(id, category, 0.0);
        println!("Create Coupon {:?}", coupon);
        self.save(coupon)
    }
}


impl_singleton_repository!(Coupon);
