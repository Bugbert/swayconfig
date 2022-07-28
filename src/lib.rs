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
                    println!("command: {}", command);
                    println!("type: {}", get_type(command));
                }
            }
        }
    }
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
    fn get_type(text: String) -> String {
        let text_str: &str = &text[..];
        let text_array = text_str.chars();
        let mut first_word_vec: Vec<char> = Vec::new();
        for i in text_array {
            if i == ' ' && first_word_vec.len() != 0 {
                break;
            } else if i == ' ' && first_word_vec.len() == 0 {
            } else if i == '#' && first_word_vec.len() == 0 {
                return String::from("comment");
            } else {
                first_word_vec.push(i);
            }
        }
        let first_word: String = first_word_vec.iter().collect();
        let first_word_str: &str = &first_word[..];
        let output: &str = match first_word_str {
            "exec" => "launch_on_start",
            "set" => "variable",
            "bindsym" => "keybind",
            "mode" => "mode",
            "gaps" => "gaps",
            "default_border" => "default_border",
            "include" => "include",
            "bar" => "swaybar",
            _ => panic!("faulty config file \"{}\"", first_word_str.to_string()),
        };
        return output.to_string();
    }
}
