use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// this is implied by unsafecell
// impl<T> !Sync for Cell<T> {}


impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    // we know this is safe becuase no one is concurrentl
    // we are not invalidating any references because we
    pub fn set(&self, value: T) {
        unsafe { *self.value.get() = value };
    }
    // we know no one else is modifying this value since
    // can mutate because !Sync, and it is executing this
    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
}

