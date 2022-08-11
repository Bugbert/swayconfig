pub mod parser {
    use std::fs;

    pub enum CommandType {
        StartupLine,
        VariableLine,
        KeybindLine,
        IncludeLine,
        CommentLine,
    }

    #[derive(Clone, Debug)]
    pub struct Variable {
        name: String,
        value: String,
    }

    #[derive(Clone, Debug)]
    pub struct Keybind {
        binding: String,
        action: String,
    }

    #[derive(Clone, Debug)]
    pub struct ConfigFile {
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
        let file_lines = read_lines(filename);
        for command in file_lines {
            let command_type: CommandType = get_type(&command);
            config_file = match command_type {
                CommandType::StartupLine => add_startup(config_file.clone(), command),
                CommandType::VariableLine => add_var(config_file.clone(), command),
                CommandType::KeybindLine => add_keybind(config_file.clone(), command),
                CommandType::IncludeLine => add_include(config_file.clone(), command),
                _ => panic!("I screwed up"),
            };
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
        let mut line: Vec<char> = Vec::new();
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
                let line_clone = line.clone();
                let line_string: String = line_clone.iter().collect();
                file_lines.push(line_string);
                line = Vec::new();
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

    fn get_type(text: &String) -> CommandType {
        let text_str: &str = &text[..];
        let text_array = text_str.chars();
        let mut first_word_vec: Vec<char> = Vec::new();
        for c in text_array {
            if c == ' ' && first_word_vec.len() != 0 {
                break;
            } else if c == ' ' && first_word_vec.len() == 0 {
            } else if c == '#' && first_word_vec.len() == 0 {
                return CommandType::CommentLine;
            } else {
                first_word_vec.push(c);
            }
        }
        let first_word: String = first_word_vec.iter().collect();
        let first_word_str: &str = &first_word[..];
        let command_type: CommandType = match first_word_str {
            "exec" => CommandType::StartupLine,
            "set" => CommandType::VariableLine,
            "bindsym" => CommandType::KeybindLine,
            "include" => CommandType::IncludeLine,
            _ => panic!("faulty config file \"{}\"", first_word_str.to_string()),
        };
        return command_type;
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
            } else if start_action {
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
