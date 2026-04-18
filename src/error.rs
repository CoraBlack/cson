//! User-facing error helpers.

use std::borrow::Cow;
use std::fmt::Display;

/// Print a concise error and exit with code 1.
pub fn fail(message: impl AsRef<str>) -> ! {
    eprintln!("cxon error: {}", message.as_ref());
    std::process::exit(1);
}

/// Install panic hook that hides Rust backtrace noise for end users.
pub fn install_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let payload = if let Some(s) = info.payload().downcast_ref::<&str>() {
            Cow::Borrowed(*s)
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            Cow::Borrowed(s.as_str())
        } else {
            Cow::Borrowed("unexpected internal error")
        };

        eprintln!("cxon error: {}", payload);
    }));
}

pub fn fail_result<T, E: Display>(result: Result<T, E>, context: impl AsRef<str>) -> T {
    match result {
        Ok(value) => value,
        Err(err) => fail(format!("{}: {}", context.as_ref(), err)),
    }
}

pub fn fail_option<T>(value: Option<T>, context: impl AsRef<str>) -> T {
    match value {
        Some(value) => value,
        None => fail(context.as_ref()),
    }
}
