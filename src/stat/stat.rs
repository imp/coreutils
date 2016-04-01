#![crate_name = "uu_stat"]

/*
 * This file is part of the uutils coreutils package.
 *
 * (c) 2016 Cyril Plisko <cyril.plisko@@mountall.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

extern crate getopts;
extern crate libc;

#[macro_use]
extern crate uucore;

use libc::{lstat, stat};
use std::env;
use std::path::{Path, PathBuf};

static NAME: &'static str = "stat";
static VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn uumain(args: Vec<String>) -> i32 {
    let mut opts = getopts::Options::new();

    opts.optflag("h", "help", "Show help and exit");
    opts.optflag("V", "version", "Show version and exit");
    opts.optopt("f", "format", "Specify format string", "FORMAT");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            show_error!("{}", f);
            show_usage(&opts);
            return 1
        }
    };

    if matches.opt_present("V") { version(); return 0 }
    if matches.opt_present("h") { show_usage(&opts); return 0 }

    if matches.free.is_empty() {
        show_error!("Missing operand: file");
        println!("Try `{} --help` for more information.", NAME);
        return 1
    }

    //let file = Path::new(&matches.free[0]);

    if matches.opt_present("f") {
        return 0
    }

    let mut buf: stat = unsafe { uninitialized() };
    let result = unsafe { lstat(matches.free[0].as_ptr() as *const c_char, &mut buf as *mut stat) };

    if result < 0 {
        crash!(1, "Cannot stat '{}': {}", matches.free[0], Error::last_os_error());
    }

    println!("{:?}", buf);
    0
}

fn version() {
    println!("{} {}", NAME, VERSION)
}

fn show_usage(opts: &getopts::Options) {
    version();
    println!("");
    println!("Usage:");
    println!("  {} [-d DIR] TO [FROM]", NAME);
    println!("  {} -V|--version", NAME);
    println!("  {} -h|--help", NAME);
    println!("");
    print!("{}", opts.usage(
            "Convert TO destination to the relative path from the FROM dir.\n\
            If FROM path is omitted, current working dir will be used.")
    );
}
