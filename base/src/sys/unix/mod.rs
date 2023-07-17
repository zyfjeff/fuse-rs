mod descriptor;
mod event;
mod poll;
use std::time::Duration;

pub use poll::EventContext;

#[macro_use]
pub mod handle_eintr;

#[macro_use]
pub mod ioctl;

pub use ioctl::*;

pub use crate::descriptor_reflection::deserialize_with_descriptors;
pub use crate::descriptor_reflection::with_as_descriptor;
pub use crate::descriptor_reflection::with_raw_descriptor;
pub use crate::descriptor_reflection::FileSerdeWrapper;
pub use crate::descriptor_reflection::SerializeDescriptors;
pub use crate::errno::Error;
pub use crate::errno::Result;
pub use crate::errno::*;
pub use descriptor::RawDescriptor;
pub use event::EventExt;
pub(crate) use event::PlatformEvent;

/// Return a timespec filed with the specified Duration `duration`.
#[allow(clippy::useless_conversion)]
pub fn duration_to_timespec(duration: Duration) -> libc::timespec {
    // nsec always fits in i32 because subsec_nanos is defined to be less than one billion.
    let nsec = duration.subsec_nanos() as i32;
    libc::timespec {
        tv_sec: duration.as_secs() as libc::time_t,
        tv_nsec: nsec.into(),
    }
}
