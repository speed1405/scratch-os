use alloc::string::String;
use alloc::vec::Vec;
use crate::println;
use crate::print;
use crate::task::keyboard::ScancodeStream;
use futures_util::stream::StreamExt;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

pub async fn run_shell() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(ScancodeSet1::new(), layouts::Us104Key,
        HandleControl::Ignore);

    let mut command = String::new();
    print!("> ");

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => {
                        if character == '\n' {
                            println!();
                            execute_command(&command);
                            command.clear();
                            print!("> ");
                        } else {
                            print!("{}", character);
                            command.push(character);
                        }
                    }
                    DecodedKey::RawKey(_) => {}
                }
            }
        }
    }
}

fn execute_command(cmd: &str) {
    match cmd.trim() {
        "" => {}
        "help" => println!("Available commands: help, hello, clear"),
        "hello" => println!("Hello from Scratch-OS!"),
        "clear" => {
            // We don't have a clear screen yet, so just print some newlines
            for _ in 0..25 { println!(); }
        }
        _ => println!("Unknown command: {}", cmd),
    }
}
