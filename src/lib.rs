pub mod parser {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

/*    struct Variable {
        name: String,
        value: String,
    }

    struct Keybind {
        binding: String,
        effect: String,
    }
    
    struct ConfigFile {
        wallpaper: String,
        vars: Vec<Variable>,
        keybinds: Vec<Keybind>,
        on_start: Vec<String>,
    }
*/
    pub fn file_to_struct(filename: String) {
        if let Ok(lines) = read_lines(filename) {
            for line in lines {
                if let Ok(command) = line {
                    println!("line: {}", command);
                }
            }
        }
    }
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}
