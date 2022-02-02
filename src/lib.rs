pub mod tools {
    use std::fs;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    pub fn open_file_as_binary(path: &Path) -> Vec<u8> {
        let mut f = File::open(&path).expect("no file found");
        let metadata = fs::metadata(&path).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");
        buffer
    }
}

use crate::tools::open_file_as_binary;
use std::path::Path;

pub fn main() {
    let png_magic: Vec<u8> = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    let test_file = Path::new("basn6a08.png");

    let file = open_file_as_binary(test_file);
    println!("PNG magic: {:#04X?}", &file[0..8]);
}
