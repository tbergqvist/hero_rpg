use std::sync::{RwLock, RwLockWriteGuard, RwLockReadGuard, Arc};

pub struct Ts<T> {
  value: Arc<RwLock<T>>
}

impl <T> Ts<T> {
  pub fn new(value: T) -> Ts<T> {
    Ts{ value: Arc::new(RwLock::new(value)) }
  }

  pub fn read(&self) -> RwLockReadGuard<T> {
    return self.value.read().unwrap()
  }

  pub fn write(&self) -> RwLockWriteGuard<T> {
    return self.value.write().unwrap()
  }

  pub fn clone(&self) -> Ts<T> {
    let value = self.value.clone();
    Ts {
      value: value
    }
  }
}