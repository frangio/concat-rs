use std::io::{Read, Result, copy, stdout};
use std::env::args_os;
use std::process::exit;
use std::ffi::OsString;
use std::fs::File;
use std::path::Path;

extern crate concat;
use concat::concat;

fn main() {
    let mut args = args_os();
    let progname = args.next().unwrap_or_else(|| OsString::from("cat"));

    let mut c = concat(args.map(InputSource::from));
    let res = copy(&mut c, &mut stdout());

    if let Err(e) = res {
        match c.current() {
            None => {
                println!("{}: {}\n",
                         AsRef::<Path>::as_ref(&progname).display(),
                         e);
            },

            Some(ref f) => {
                println!("{}: {}: {}\n",
                         AsRef::<Path>::as_ref(&progname).display(),
                         f.path().display(),
                         e);
            },
        };

        exit(1);
    }
}

struct InputSource {
    path: OsString,
    file: Option<File>,
}

impl InputSource {
    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }
}

impl From<OsString> for InputSource {
    fn from(path: OsString) -> InputSource {
        InputSource {
            path: path,
            file: None,
        }
    }
}

impl Read for InputSource {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if self.file.is_none() {
            self.file = Some(try!(File::open(&self.path)));
        }

        self.file.as_mut().unwrap().read(buf)
    }
}
