use std::{
    alloc,
    alloc::Layout,
    mem,
    ptr::{self, NonNull},
    slice,
};

const INIT_CAPACITY: usize = 4;

pub struct MyVec<T> {
    // NonNull is similar to *const T (i.e. a pointer) but is non-zero and covariant. ptr is the
    // pointer to the first element in the vec.
    ptr: NonNull<T>,
    len: usize,
    // Number of elements that could fit in the heap allocation that Vec made, without needing for
    // a new allocation.
    capacity: usize,
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len));
            let layout = Layout::from_size_align_unchecked(
                mem::size_of::<T>() * self.capacity,
                mem::align_of::<T>(),
            );
            alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout)
        }
    }
}

impl<T> Default for MyVec<T> {
    // Creates an empty `MyVec<T>`
    fn default() -> Self {
        Self::new()
    }
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }

    fn resize(&mut self) {
        let new_capacity = match self.capacity {
            0 => INIT_CAPACITY,
            n => n.checked_mul(2).expect("Capacity overflow"),
        };

        let mem_layout = Layout::array::<T>(new_capacity).expect("Could not create layout");
        // SAFETY: alloc() expects a sized layout, and our layout is hardcoded to be sized
        // (at least INIT_CAPACITY * size_of::<T>) and size_of::<T> is not 0.
        let ptr = unsafe { alloc::alloc(mem_layout) } as *mut T;
        let mut ptr = NonNull::new(ptr).expect("Could not create non-null pointer");
        mem::swap(&mut self.ptr, &mut ptr);

        if self.capacity != 0 {
            // SAFETY: Guaranteed that both ptr will never be null or zero-sized by using
            // ptr::NonNull.
            unsafe {
                ptr.as_ptr().copy_to(self.ptr.as_ptr(), self.len);
                let old_mem_layout =
                    Layout::array::<T>(self.capacity).expect("Could not create layout");
                alloc::dealloc(ptr.as_ptr() as *mut u8, old_mem_layout)
            }
        };

        self.capacity = new_capacity;
    }

    pub fn push(&mut self, item: T) {
        assert_ne!(mem::size_of::<T>(), 0, "No zero sized types");

        if self.capacity == 0 || self.len == self.capacity {
            self.resize();
        }

        // Panic if multiplication (self.len * size_of::<T>) results in overflow.
        let offset = self
            .len
            .checked_mul(mem::size_of::<T>())
            .expect("Cannot reach memory location");
        assert!(offset < isize::MAX as usize, "Offset overflow isize");

        // SAFETY:
        //
        // For `add`:
        //
        // We have made sure that there is enough space in the layout by
        // reallocating when capacity is full.
        //
        // We have also made sure that the computed offset is always less
        // than the max isize. (so it would never overflow an isize)
        //
        // For `write`:
        //
        // We have made sure that the memory is correctly
        // sized and alligned by using Layout when allocating.
        unsafe { self.ptr.as_ptr().add(self.len).write(item) };
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index > (self.len - 1) {
            return None;
        }

        let item = unsafe { self.ptr.as_ptr().add(index).as_ref() };
        item
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocate() {
        let mut vec = MyVec::new();
        vec.push(69usize);

        assert_eq!(vec.capacity(), 4);
        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn allocate_1000() {
        let mut vec = MyVec::new();

        for i in 0..1000 {
            vec.push(i);
        }

        assert_eq!(vec.len(), 1000);
    }

    #[test]
    fn grow_size() {
        let mut vec = MyVec::new();
        vec.push(69usize);
        vec.push(69);
        vec.push(69);
        vec.push(69);
        vec.push(69);

        assert_eq!(vec.capacity(), 8);
        assert_eq!(vec.len(), 5);
    }

    #[test]
    fn get() {
        let mut vec = MyVec::new();
        vec.push(1usize);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        vec.push(5);
        vec.push(6);
        assert_eq!(vec.get(5), Some(&6));
        assert_eq!(vec.get(6), None);
    }

    #[test]
    fn get_1_000() {
        let mut vec = MyVec::new();

        for i in 0..1000 {
            vec.push(i);
        }

        for i in 0..1000 {
            assert_eq!(vec.get(i), Some(&i));
        }
    }

    #[test]
    fn get_1_000_000() {
        let mut vec = MyVec::new();

        for i in 0..1_000_000 {
            vec.push(i);
        }

        assert_eq!(vec.len(), 1_000_000);

        for i in 0..1_000_000 {
            assert_eq!(vec.get(i), Some(&i));
        }

        assert_eq!(vec.get(1_000_001), None);
    }
}
