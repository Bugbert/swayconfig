pub mod parser {
    use std::fs;

    pub fn stringify(file: String) -> String {
        return fs::read_to_string(file)
            .expect("Something went wrong reading the file");
    }
}
