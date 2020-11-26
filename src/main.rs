use memmap::Mmap;
use nix::unistd::ftruncate;
use std::fs::{remove_file, File};
use std::os::unix::io::AsRawFd;
use std::{thread, time};

fn main() {
    let path = "buffer.bin";
    let f = File::create(&path).unwrap();

    let fd = f.as_raw_fd();
    ftruncate(fd, 1024 * 1024 * 1024).unwrap();

    let f = File::open(&path).unwrap();
    let mmap = unsafe { Mmap::map(&f).expect("Unable to mmap") };
    println!("Scanned {} windows", mmap.windows(1024 * 1024).count());

    let hour = time::Duration::from_secs(3600);
    thread::sleep(hour);

    drop(mmap);
    remove_file(&path).unwrap();
}
