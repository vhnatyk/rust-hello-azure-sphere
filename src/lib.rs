#![no_std]
#![feature(no_std, alloc_error_handler)]

extern crate alloc;

mod allocator;


#[cfg(not(test))]
#[global_allocator]
static GLOBAL: allocator::MyAllocator = allocator::MyAllocator;

mod log;
mod woo;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
