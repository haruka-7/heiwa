use std::fs::read_to_string;

pub fn read_file(file_path: &String) -> String {
    tracing::debug!("Read file with path {}", file_path);
    read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Should have been able to read the file : {}", file_path))
}
