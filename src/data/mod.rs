#[doc(hidden)] pub mod core;
#[doc(hidden)] pub mod from;
#[doc(hidden)] pub mod into;
#[doc(hidden)] pub mod ops;
#[doc(hidden)] pub mod seq;
#[doc(inline)] pub use core::{Data, DataIterator, ToData};

#[doc(inline)] pub use seq::{DataSeq, DataSeqIterator};
