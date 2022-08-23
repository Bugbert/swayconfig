#![allow(dead_code)]

pub mod parser {
    use std::fs;

    #[derive(Debug)]
    pub enum Command {
        StartupLine(u8, Vec<String>),
        VariableLine(u8, Vec<String>),
        KeybindLine(u8, Vec<String>),
        IncludeLine(u8, Vec<String>),
        CommentLine(u8, Vec<String>),
    }

    #[derive(Debug)]
    pub enum Element {
        Startup(String),
        Variable(String, String),
        Keybind(String, String),
        Include(String),
        Comment(String),
    }

    pub fn file_to_struct(filename: String) {
        let mut config_file: Vec<Element> = Vec::new();
        let file_lines = read_lines(filename);
        for command_line in file_lines {
            let command: Command = get_type(split_line(command_line));
            config_file.push(command_to_element(command));
        }
        println!("{:?}", config_file);
    }

    pub fn read_lines(filename: String) -> Vec<String> {
        let contents: String = fs::read_to_string(filename)
            .expect("Something went wrong reading the config file");
        let contents_str: &str = &contents[..];
        let contents_array = contents_str.chars();
        let mut file_lines: Vec<String> = Vec::new();
        let mut backslash: bool = false;
        let mut bracket: bool = false;
        let mut in_block: bool = false;
        let mut line: String = String::new();
        for c in contents_array {
            if c == '\\' {
                backslash = true;
            } else if c == '\n' && backslash {
                line.push(' ');
                backslash = false;
            } else if c == '{' {
                line.push('{');
                bracket = true;
            } else if c == '}' && in_block {
                line.push('}');
                in_block = false;
            } else if c == '\n' && bracket {
                line.push(' ');
                in_block = true;
                bracket = false;
            } else if c == '\n' && line.len() != 0 && !in_block {
                file_lines.push(line);
                line = String::new();
            } else if c == '\n' && line.len() == 0 {
            } else if backslash == true {
                line.push('\\');
                line.push(c);
            } else {
                line.push(c);
            }
        }
        return file_lines;
    }

    pub fn get_type(words: (u8, Vec<String>)) -> Command {
        println!("{:?}", &words);
        let first_word: &str = &words.1[0][..];
        if first_word.chars().next().unwrap() == '#' {
            return Command::CommentLine(words.0, words.1);
        }
        match first_word {
            "set" => Command::VariableLine(words.0, words.1),
            "exec" => Command::StartupLine(words.0, words.1),
            "bindsym" => Command::KeybindLine(words.0, words.1),
            "include" => Command::IncludeLine(words.0, words.1),
            _ => panic!("faulty config file"),
        }
    }

    pub fn split_line(line: String) -> (u8, Vec<String>) {
        println!("{}", &line);
        let line_str: &str = &line[..];
        let line_array = line_str.chars();
        let mut tabs: u8 = 0;
        let mut current_word: Vec<char> = Vec::new();
        let mut words: Vec<String> = Vec::new();
        let mut in_dquotes: bool = false;
        let mut in_squotes: bool = false;
        for c in line_array {
            if c == '"' && !in_dquotes && !in_squotes {
                current_word.push(c);
                in_dquotes = true;
            } else if c == '\'' && !in_dquotes && !in_squotes {
                current_word.push(c);
                in_squotes = true;
            } else if c == '"' && in_dquotes {
                current_word.push(c);
                let word: String = current_word.iter().collect();
                words.push(word);
                in_dquotes = false;
                current_word = Vec::new();
            } else if c == '\'' && in_squotes {
                current_word.push(c);
                let word: String = current_word.iter().collect();
                words.push(word);
                in_squotes = false;
                current_word = Vec::new();
            } else if c == ' ' && current_word.len() != 0 && !in_dquotes && !in_squotes {
                let word: String = current_word.iter().collect();
                words.push(word);
                current_word = Vec::new();
            } else if c == '\t' && current_word.len() == 0 && words.len() == 0 {
                tabs += 1;
            } else {
                current_word.push(c);
            }
        }
        return (tabs, words);
    }

    pub fn command_to_element(command: Command) -> Element {
        match command {
            Command::StartupLine(_, words) => make_startup(words),
            Command::IncludeLine(_, words) => Element::Include(words.get(1).unwrap()[..]
                .to_string()),
            Command::KeybindLine(_, words) => make_keybind(words),
            Command::VariableLine(_, words) => make_variable(words),
            Command::CommentLine(_, words) => make_comment(words),
        }
    }

    pub fn make_startup(mut words: Vec<String>) -> Element {
        words.remove(0); // removes the "exec"
        let mut action: String = String::new();
        for word in words {
            action.push_str(&word[..]);
        }
        return Element::Startup(action);
    }

    pub fn make_variable(mut words: Vec<String>) -> Element {
        words.remove(0);
        let var_name_ref = words.get(0).unwrap();
        let var_name = var_name_ref[..].to_string();
        words.remove(0);
        let mut value = String::new();
        for word in words {
            value.push_str(&word[..]);
        }
        return Element::Variable(var_name, value);
    }

    pub fn make_keybind(mut words: Vec<String>) -> Element {
        words.remove(0);
        let keybind_ref = words.get(0).unwrap();
        let keybind = keybind_ref[..].to_string();
        words.remove(0);
        let mut action = String::new();
        for word in &words {
            action.push_str(&word[..]);
        }
        return Element::Keybind(keybind, action);
    }

    pub fn make_comment(words: Vec<String>) -> Element {
        let mut comment = String::new();
        for word in words {
            comment.push_str(&word[..]);
        }
        return Element::Comment(comment);
    }
}
