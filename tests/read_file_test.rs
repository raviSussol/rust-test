#[cfg(test)]
mod test_read_file {
    use rust_test::read_file::{read_file_as_buffer_string, read_file_as_string};

    #[test]
    pub fn test_read_file_as_buffer_string_success() {
        let file_path = "/Users/sussol/Downloads/HOSDID counts copy.txt";
        let contents = read_file_as_buffer_string(file_path).unwrap();
        assert_ne!(contents, "");
    }

    #[test]
    pub fn test_read_file_as_buffer_string_error() {
        let file_path = "";
        let result = read_file_as_buffer_string(file_path).map_err(|e| e.kind());
        let expected = Err(std::io::ErrorKind::NotFound);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_read_file_as_string_success() {
        let file_path = "/Users/sussol/Downloads/HOSDID counts copy.txt";
        let contents = read_file_as_string(file_path).unwrap();
        assert_ne!(contents, "");
    }

    #[test]
    pub fn test_read_file_as_string_error() {
        let file_path = "";
        let result = read_file_as_string(file_path).map_err(|e| e.kind());
        let expected = Err(std::io::ErrorKind::NotFound);
        assert_eq!(result, expected);
    }
}
