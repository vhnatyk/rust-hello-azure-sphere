#![cfg_attr(not(std), no_std)]
#![cfg_attr(not(std), feature(alloc_error_handler))]

#[cfg(feature = "std")]
#[macro_use]
extern crate std;

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
mod allocator;


#[cfg(not(feature = "std"))]
#[cfg_attr(not(std), global_allocator)]
static GLOBAL: allocator::MyAllocator = allocator::MyAllocator;

mod log;
#[macro_use]
mod woo;

#[cfg(test)]
mod tests {   
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
