
#![cfg_attr(feature="xcheck-with-dlsym", feature(const_fn))]
#![cfg_attr(feature="xcheck-with-dlsym", feature(const_ptr_null_mut))]
#![cfg_attr(feature="xcheck-with-dlsym", feature(libc))]
#![cfg_attr(feature="xcheck-with-weak", feature(linkage))]

pub mod xcheck;
pub mod djb2;
pub mod hash;

