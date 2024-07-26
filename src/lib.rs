//#![warn(missing_docs)]

use cfg_if::cfg_if;

mod error;
pub use error::Error;

mod event;
// pub use event::Event;

mod constants;
// pub use constants::;

mod builder;
// pub use builder::SocketBuilder;

// Plumbing for making it work with and without tracing
cfg_if! {
    if #[cfg(feature = "tracing")] {
        #[allow(unused_imports)]
        use tracing::{trace, debug, info, warn, error};
    } else {
        mod dummy_tracing;
    }
}
