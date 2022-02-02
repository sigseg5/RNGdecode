use std::path::Path;

mod tools {
    use std::fs;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    pub(crate) fn open_file_as_binary(path: &Path) -> Vec<u8> {
        let mut f = File::open(&path).expect("no file found");
        let metadata = fs::metadata(&path).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");
        buffer
    }
}

fn main() {
    let file = tools::open_file_as_binary(Path::new("basn6a08.png"));
    println!("content: {:?}", file);
}
