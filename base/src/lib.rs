pub mod descriptor;
pub mod descriptor_reflection;
mod errno;
mod event;
pub mod sys;
mod wait_context;

pub use errno::errno_result;
pub use errno::Error;
pub use errno::Result;
pub use event::Event;
pub use event::EventWaitResult;

pub use wait_context::EventToken;
pub use wait_context::EventType;
pub use wait_context::TriggeredEvent;
pub use wait_context::WaitContext;

pub use platform::deserialize_with_descriptors;
pub use platform::FileSerdeWrapper;
pub use platform::SerializeDescriptors;
pub use sys::platform;

cfg_if::cfg_if! {
    if #[cfg(unix)] {
        pub use sys::unix;
        pub use platform::EventExt;
    } else if #[cfg(windows)] {
    } else {
        compile_error!("Unsupported platform");
    }
}

pub use platform::RawDescriptor;

pub use crate::descriptor::AsRawDescriptor;
pub use crate::descriptor::AsRawDescriptors;
pub use crate::descriptor::Descriptor;
pub use crate::descriptor::FromRawDescriptor;
pub use crate::descriptor::IntoRawDescriptor;
pub use crate::descriptor::SafeDescriptor;

pub use platform::with_as_descriptor;
pub use platform::with_raw_descriptor;
pub use platform::EventContext;

pub use platform::ioctl::ioctl;
pub use platform::ioctl::ioctl_with_mut_ptr;
pub use platform::ioctl::ioctl_with_mut_ref;
pub use platform::ioctl::ioctl_with_ptr;
pub use platform::ioctl::ioctl_with_ref;
pub use platform::ioctl::ioctl_with_val;
pub use platform::ioctl::IoctlNr;
