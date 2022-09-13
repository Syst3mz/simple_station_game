use inquire::{InquireError, Select, Text};
use crate::game_system::*;
use strum::VariantNames;
use strum_macros::{EnumString, EnumVariantNames};
use std::str::FromStr;
use std::{thread, time, usize};
use std::io::Write;
use std::time::Duration;
use rand::{Rng, thread_rng};
use colored::*;

#[derive(Debug, PartialEq, EnumString, EnumVariantNames)]
enum MainMenuChoice {
    Start,
    Quit
}
#[derive(PartialEq, EnumString, EnumVariantNames)]
enum GameOptions {
    Status,
    Fix,
    #[strum(serialize = "Do Science")]
    DoScience,
    #[strum(serialize = "Log Off For The Night")]
    LogoffForTheNight,
    Quit,
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
            Err(InquireError::OperationInterrupted) => break,
            Err(_) => panic!("A unexpected error has occurred when parsing your choice")
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
fn get_list_of_modules<'a>(player: &Player) -> Vec<String> {
    let mut v: Vec<String> = Vec::<String>::new();
    let mut index = 1;
    for module in &player.station.modules {
        v.push(format!("{}) {}, STATUS: {} ({:.4})\n", index, module.module_type.to_string(),
                        if module.broken {
                            "BROKEN".red()
                        } else {
                            "NOMINAL".green()
                        }, module.breakdown_bias.to_string().yellow()));
        index +=1 ;
    }
    v
}

fn get_status_string(player: &Player) -> String {
    let mut ret = format!("Day: {}, done {} science\n",
             player.days_survived.to_string().yellow(),
             format!("{:.2}", player.science_done).yellow()
    );
    ret.push_str(&format!("Station: {}\n", player.station.name.to_string()));
    for module in &player.station.modules {
        ret.push_str(&format!("\t{}, STATUS: {} ({:.4})\n", module.module_type.to_string(),
                             if module.broken {"BROKEN".red()} else {"NOMINAL".green()}, module.breakdown_bias.to_string().yellow()));
    }
    ret.push_str(&format!("FixIT Bot charge: {}\n", player.pips_left_today.to_string().yellow()));
    ret
}

fn print_no_pips_msg() {
    println!("{} {}", "ERR".red(), "FixIT Bot lacks enough charge to do anything more today")
}

fn start_game() {
    let player_name = get_valid_string("What is your username cosmonaut?");
    print!("Welcome {}!\nPassword: ", player_name);
    for c in "**********\n".chars() {
        print!("{}", c);
        std::io::stdout().flush();
        thread::sleep(time::Duration::from_millis(thread_rng().gen_range(75..140)));
    }

    // here I would handle loading a save if I implement it

    let mut player = Player::new(player_name, Station::get_random_station());

    loop {
        let game_options= Select::new("ENTER COMMAND", Vec::from(GameOptions::VARIANTS)).prompt();
        match game_options {
            Ok(answer) => match GameOptions::from_str(answer) {
                Ok(answer) => {
                    match answer {
                        GameOptions::Quit => break,
                        GameOptions::Status => {
                            for line in get_status_string(&player).lines(){
                                println!("{}", line);
                                thread::sleep(Duration::from_millis(thread_rng().gen_range(50..240)))
                            }
                        }
                        GameOptions::LogoffForTheNight => {
                            player.days_survived += 1;
                            player.pips_left_today = 3;
                            player.station.tick();
                        }
                        GameOptions::DoScience => {
                            if player.pips_left_today > 0 {
                                println!("{} science done!", player.do_science().to_string().yellow());
                            }
                            else {
                                print_no_pips_msg();
                            }
                        }
                        GameOptions::Fix => {
                            if player.pips_left_today > 0 {
                                let v = Select::new("What should be fixed?",
                                                    get_list_of_modules(&player)).prompt();
                                match v {
                                    Ok(value) => {
                                        let num = value.split(')').next().unwrap();
                                        match usize::from_str(num) {
                                            Ok(num) => {
                                                player.fix_module_at_index(num-1)
                                            }
                                            _ => panic!("An unexpected error has occurred when parsing your input.")
                                        }
                                    }
                                    Err(e) => match e {
                                        InquireError::OperationInterrupted => break,
                                        _ => panic!("An error occurred while parsing your choice.")
                                    }
                                }
                            }
                            else {
                                print_no_pips_msg();
                            }
                        }
                    }
                }
                _ => panic!("An error occurred while parsing your choice.")
            }
            Err(InquireError::OperationInterrupted) => break,
            Err(_) => panic!("An error has occurred when selecting your choice")
        }
    }
}