use crate::commands;
use crate::println;

pub struct Command {
    pub name: &'static str,
    pub func: fn(&str),
}

pub static COMMAND_REGISTRY: &[Command] = &[
    Command {
        name: "clear",
        func: commands::clear,
    },
    Command {
        name: "echo",
        func: commands::echo,
    },
    Command {
        name: "help",
        func: commands::help,
    },
    Command {
        name: "info",
        func: commands::info,
    },
];

pub fn execute(input: &str) {
    // Убираем пробелы и символы переноса строки по краям
    let input = input.trim();
    if input.is_empty() {
        return;
    }

    let mut parts = input.splitn(2, ' ');
    let cmd_name = parts.next().unwrap_or("");
    let args = parts.next().unwrap_or("");

    match COMMAND_REGISTRY.binary_search_by(|cmd| cmd.name.cmp(cmd_name)) {
        Ok(index) => {
            (COMMAND_REGISTRY[index].func)(args);
        }
        Err(_) => {
            println!("Unknown command: {}", cmd_name);
        }
    }
}
