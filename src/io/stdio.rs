use std::fmt;

use log::Level;

use crate::ffi;

const LOG_LEVEL_ERROR: u8 = 1;
const LOG_LEVEL_WARN: u8 = 2;
const LOG_LEVEL_INFO: u8 = 3;
const LOG_LEVEL_DEBUG: u8 = 4;
const LOG_LEVEL_TRACE: u8 = 5;

macro_rules! print {
    ($($arg:tt)*) => {
        $crate::ffi::io::print(::std::format_args!($($arg)*), false)
    }
}

macro_rules! println {
    ($($arg:tt)*) => {
        $crate::ffi::io::print(::std::format_args!($($arg)*), true)
    }
}

macro_rules! eprint {
    ($($arg:tt)*) => {
        $crate::ffi::io::eprint(::std::format_args!($($arg)*), false)
    }
}

macro_rules! eprintln {
    ($($arg:tt)*) => {
        $crate::ffi::io::eprint(::std::format_args!($($arg)*), true)
    }
}

pub fn init_logger() {
    static LOGGER: Logger = Logger;

    if log::set_logger(&LOGGER).is_err() {
        panic!("attempted to initialize logger twice")
    }
}

pub fn print(args: fmt::Arguments<'_>, new_line: bool) {
    print_internal(args, new_line, ffi::io::print);
}

pub fn eprint(args: fmt::Arguments<'_>, new_line: bool) {
    print_internal(args, new_line, ffi::io::eprint)
}

fn print_internal(
    args: fmt::Arguments<'_>,
    new_line: bool,
    f: unsafe extern "C" fn(*const u8, usize, bool),
) {
    let s;
    let (ptr, len) = match args.as_str() {
        Some(s) => (s.as_ptr(), s.len()),
        None => {
            s = args.to_string();
            (s.as_ptr(), s.len())
        }
    };

    unsafe { f(ptr, len, new_line) }
}

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() >= log::max_level()
    }

    fn log(&self, record: &log::Record) {
        if record.level() >= log::max_level() {
            let msg;
            let level = match record.level() {
                Level::Error => LOG_LEVEL_ERROR,
                Level::Warn => LOG_LEVEL_WARN,
                Level::Info => LOG_LEVEL_INFO,
                Level::Debug => LOG_LEVEL_DEBUG,
                Level::Trace => LOG_LEVEL_TRACE,
            };

            let args = record.args();
            let (ptr, len) = match args.as_str() {
                Some(s) => (s.as_ptr(), s.len()),
                None => {
                    msg = args.to_string();
                    (msg.as_ptr(), msg.len())
                }
            };

            unsafe {
                ffi::io::log(level, ptr, len);
            }
        }
    }

    fn flush(&self) {}
}
