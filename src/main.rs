use clap::{App, Arg};
use std::fs;
use std::io::{Read, Write};
use std::path;
use std::mem;
use std::slice;
#[repr(C)]
struct Header {
    magic: [u8; 16],
    len: u32
}

impl Header {
    fn new(len :u32) -> Self {
        Self {
            magic: *b"Shaoxi2010Loader",
            len
        }
    }
    fn is_ok(&self) -> bool {
        if self.magic == *b"Shaoxi2010Loader" {
            true
        } else {
            false
        }
    }
    fn total(&self) -> u32 {
        self.len + (mem::size_of::<Header>()) as u32
    }

    fn tobytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const _ as *const u8, mem::size_of::<Header>())
        }
    }
}

fn main(){
    let matches = App::new("loaderlz4gen")
        .arg(Arg::with_name("out")
            .required(true)
            .value_name("file")
            .short("o"))
        .arg(Arg::with_name("input")
            .required(true))
        .help("auto gen loader compress img")
        .get_matches();

    let out = matches.value_of("out").unwrap();
    let input = matches.value_of("input").unwrap();

    let input = path::PathBuf::from(input);
    let out = path::PathBuf::from(out);

    let mut f = fs::File::open(input).unwrap();

    let mut data = Vec::new();
    f.read_to_end(&mut data).unwrap();
    let compressed = lz4_flex::compress_prepend_size(&data);

    let header = Header::new(compressed.len() as u32);

    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(out).unwrap();

    f.write(header.tobytes()).unwrap();
    f.write(&compressed).unwrap();


}
