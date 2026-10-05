//! Zero-cost no-op telemetry stub for minimal Zed.
use std::any::Any;

#[macro_export]
macro_rules! event {
    ($name:expr) => {{
        let _ = &$name;
    }};
    ($name:expr, $($key:ident $(= $value:expr)?),+ $(,)?) => {{
        let _ = &$name;
        $(
            $crate::serialize_property!($key $(= $value)?);
        )+
    }};
}

#[macro_export]
macro_rules! serialize_property {
    ($key:ident) => {
        let _ = &$key;
    };
    ($key:ident = $value:expr) => {
        let _ = &$value;
    };
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Event;

pub fn send_event(_event: impl Any) {}

pub fn init<T>(_tx: T) {}
