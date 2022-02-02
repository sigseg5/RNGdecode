#[cfg(test)]
mod tests {
    use rngdecode_lib::tools::open_file_as_binary;
    use std::path::Path;

    #[test]
    fn test_open_file_as_binary() {
        // png magic from https://en.wikipedia.org/wiki/List_of_file_signatures
        let png_magic: Vec<u8> = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        let test_file = Path::new("basn6a08.png");
        let file = open_file_as_binary(&test_file);
        assert_eq!(png_magic, &file[0..8]);
    }
}
