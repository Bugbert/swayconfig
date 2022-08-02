pub mod parser {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    #[derive(Clone)]
    struct Variable {
        name: String,
        value: String,
    }

    #[derive(Clone)]
    struct Keybind {
        binding: String,
        action: String,
    }

    #[derive(Clone)]
    struct ConfigFile {
        vars: Vec<Variable>,
        keybinds: Vec<Keybind>,
        startup: Vec<String>,
        include: Vec<String>,
    }

    pub fn file_to_struct(filename: String) {
        let mut config_file = ConfigFile {
            vars: Vec::new(),
            keybinds: Vec::new(),
            startup: Vec::new(),
            include: Vec::new(),
        };
        if let Ok(lines) = read_lines(filename) {  // If read_lines sucseeds, lines is an array
            for line in lines {
                if let Ok(command) = line {
                    let command_type: String = get_type(&command);
                    let type_str: &str = &command_type[..];
                    config_file = match type_str {
                        "startup" => add_startup(config_file.clone(), command),
                        "variable" => add_var(config_file.clone(), command),
                        "keybind" => add_keybind(config_file.clone(), command),
                        "include" => add_include(config_file.clone(), command),
                        _ => panic!("I fucked up"),
                    };
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

    fn get_type(text: &String) -> String {
        let text_str: &str = &text[..];
        let text_array = text_str.chars();
        let mut first_word_vec: Vec<char> = Vec::new();
        for c in text_array {
            if c == ' ' && first_word_vec.len() != 0 {
                break;
            } else if c == ' ' && first_word_vec.len() == 0 {
            } else if c == '#' && first_word_vec.len() == 0 {
                return String::from("comment");
            } else {
                first_word_vec.push(c);
            }
        }
        let first_word: String = first_word_vec.iter().collect();
        let first_word_str: &str = &first_word[..];
        let output: &str = match first_word_str {
            "exec" => "startup",
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

    fn add_var(mut config_file: ConfigFile, command: String) -> ConfigFile {
        let cmd_str: &str = &command[..];
        let cmd_array = cmd_str.chars();
        let mut name_vec: Vec<char> = Vec::new();
        let mut value_vec: Vec<char> = Vec::new();
        let mut start_name: bool = false;
        let mut start_value: bool = false;
        for c in cmd_array {
            if c == '$' && !start_name && !start_value {
                start_name = true;
            } else if start_name && c == ' ' {
                start_name = false;
                start_value = true;
            } else if start_name {
                name_vec.push(c);
            } else if start_value {
                value_vec.push(c);
            }
        }
        let name: String = name_vec.iter().collect();
        let value: String = value_vec.iter().collect();
        let variable = Variable {
            name: name,
            value: value,
        };
        config_file.vars.push(variable);
        return config_file;
    }

    fn add_keybind(mut config_file: ConfigFile, command: String) -> ConfigFile {
        let cmd_str: &str = &command[..];
        let cmd_array = cmd_str.chars();
        let mut bind_vec: Vec<char> = Vec::new();
        let mut action_vec: Vec<char> = Vec::new();
        let mut on_command: bool = false;
        let mut start_bind: bool = false;
        let mut start_action: bool = false;
        for c in cmd_array {
            if c != ' ' && !on_command && !start_bind && !start_action {
                on_command = true;
            } else if c == ' ' && on_command {
                on_command = false;
                start_bind = true;
            } else if c != ' ' && start_bind {
                bind_vec.push(c);
            } else if c == ' ' && start_bind {
                start_bind = false;
                start_action = true;
            } else if start_action {
                action_vec.push(c);
            }
        }
        let binding: String = bind_vec.iter().collect();
        let action: String = action_vec.iter().collect();
        let keybind = Keybind {
            binding: binding,
            action: action,
        };
        config_file.keybinds.push(keybind);
        return config_file;
    }

    fn add_startup(mut config_file: ConfigFile, command: String) -> ConfigFile {
        let cmd_str: &str = &command[..];
        let cmd_array = cmd_str.chars();
        let mut action_vec: Vec<char> = Vec::new();
        let mut on_command: bool = false;
        let mut start_action: bool = false;
        for c in cmd_array {
            if c != ' ' && !on_command && !start_action {
                on_command = true;
            } else if c == ' ' && on_command {
                on_command = false;
                start_action = true;
            } else if c != ' ' && start_action {
                action_vec.push(c);
            }
        }
        let action: String = action_vec.iter().collect();
        config_file.startup.push(action);
        return config_file;
    }

    fn add_include(mut config_file: ConfigFile, command: String) -> ConfigFile {
        let cmd_str: &str = &command[..];
        let cmd_array = cmd_str.chars();
        let mut inc_vec: Vec<char>= Vec::new();
        let mut on_command: bool = false;
        let mut start_inc: bool = false;
        for c in cmd_array {
            if c != ' ' && !on_command && !start_inc {
                on_command = true;
            } else if c == ' ' && on_command {
                on_command = false;
                start_inc = true;
            } else if c != ' ' && start_inc {
                inc_vec.push(c);
            }
        }
        let inc: String = inc_vec.iter().collect();
        config_file.include.push(inc);
        return config_file;
    }
}
