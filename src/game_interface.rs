use inquire::{InquireError, Select, Text};
use crate::game_system::*;
use strum::VariantNames;
use strum_macros::{EnumString, EnumVariantNames};
use std::str::FromStr;
use std::{thread, time};
use std::io::Write;
use rand::{random, Rng, RngCore, thread_rng};

#[derive(Debug, PartialEq, EnumString, EnumVariantNames)]
enum MainMenuChoice {
    Start,
    Quit
}
#[derive(PartialEq, EnumString, EnumVariantNames)]
enum GameOptions {
    Status,
    Quit
}

pub fn begin_game_loop() {
    loop {
        let main_menu_result = Select::new("Main Menu",
                                           Vec::from(MainMenuChoice::VARIANTS)).prompt();
        match main_menu_result {
            Ok(value) => {
                match MainMenuChoice::from_str(value).unwrap() {
                    MainMenuChoice::Start => start_game(),
                    MainMenuChoice::Quit => break,
                }
            }
            Err(_) => println!("A magic error has happened")
        }
    }
}

fn get_valid_string(prompt: &str) -> String{
    loop {
        match Text::new(prompt).prompt() {
            Ok(name) => return name,
            Err(e) => {
                match e {
                    InquireError::OperationInterrupted => panic!("Application exited by CTRL+C"),
                    _ => println!("I don't know how you got here, but please enter a valid string.")
                }
            }
        }
    }
}

fn start_game() {
    let player_name = get_valid_string("What is your username cosomonaut?");
    print!("Welcome {}!\nPassword: ", player_name);
    std::io::stdout().flush();
    for c in "**********\n".chars() {
        print!("{}", c);
        std::io::stdout().flush();
        thread::sleep(time::Duration::from_millis(thread_rng().gen_range(75..140)));
    }

    let game_options= Select::new("ENTER COMMAND", GameOptions::VARIANTS).prompt();
    match game_options {
        Ok(answer) => match GameOptions::from_str(answer) {
            GameOptions::Status => {

            }
            GameOptions::Quit => {}
        }
        Err(_) => return
    }
}