#![feature(cfg_overflow_checks)]
pub mod errors;
pub mod io;
pub mod models;

pub use errors::*;
pub use io::*;
pub use models::*;

pub mod hash;
pub use hash::*;

pub mod traits;
pub use traits::*;

pub mod cli;
pub use cli::*;

#[macro_export]
macro_rules! trace_info {
    ( $o:literal, $l:literal, $($parts:expr),* ) => {
        {
            use tracing::{event, span, Level};
            let span = span!(Level::INFO, $o);
            let _enter_ = span.enter();
            event!(Level::INFO, $l, $($parts,)*);
        }
    };
    ( $o:literal, $l:literal ) => {
        use tracing::{event, span, Level};
        let span = span!(Level::INFO, $o);
        let _enter_ = span.enter();
        event!(Level::INFO, $l);
    };
}
