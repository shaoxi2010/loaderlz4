use std::alloc::handle_alloc_error;
use clap::{App, Arg};
use std::fs;
use std::io::{Read, Write};
use std::path;
use std::mem;
use std::mem::size_of;
use std::slice;
#[repr(C)]
#[repr(align(2048))]
struct Header {
    magic: [u8; 16],
    compressed: u32,
    len: u32,
    load: u32,
    size: u32
}

impl Header {
    fn new() -> Self {
        Self {
            magic: *b"Shaoxi2010Loader",
            compressed: 0,
            len: 0,
            load: 0x8000_0000,
            size: 0
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
use crc::{Crc, Algorithm, CRC_32_ISCSI};

fn main(){
    let matches = App::new("loaderlz4gen")
        .arg(Arg::with_name("out")
            .required(true)
            .value_name("file")
            .short("o"))
        .arg(Arg::with_name("loadaddr")
            .required(false)
            .value_name("addr")
            .short("-l"))
        .arg(Arg::with_name("input")
            .required(true))
        .help("auto gen loader compress img")
        .get_matches();

    let out = matches.value_of("out").unwrap();
    let input = matches.value_of("input").unwrap();
    let addr = matches.value_of("loadaddr").unwrap_or("0xffffffff");
    let addr= u32::from_str_radix(addr.trim_start_matches("0x"), 16).unwrap();

    let input = path::PathBuf::from(input);
    let out = path::PathBuf::from(out);

    let mut f = fs::File::open(input).unwrap();

    let mut data = Vec::new();
    f.read_to_end(&mut data).unwrap();
    println!("CRC:{:08x}", Crc::<u32>::new(&CRC_32_ISCSI).checksum(&data));
    let compressed = lz4_flex::block::compress_prepend_size(&data);
    let mut header = Header::new();
    let mut f = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(out).unwrap();

    if addr == 0xffffffff {
        header.len = data.len() as u32;
        header.size = 0;
        f.write(header.tobytes()).unwrap();
        f.write(&data).unwrap();
    } else {
        header.compressed = 1;
        header.len = compressed.len() as u32;
        header.load = addr;
        header.size = data.len() as u32;
        f.write(header.tobytes()).unwrap();
        f.write(&compressed).unwrap();
    }
}
