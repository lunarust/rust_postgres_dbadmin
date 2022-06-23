use std::env;
use clap::{App, Arg};
use std::io::*;
mod pgpass;
mod users;
//use std::process::Command;
use self::pgpass::*;

fn main() {
    println!("Hi there!");
    let mut myusername: String;
    let mut myaction: String;
    let args: Vec<String> = env::args().collect();
    myaction = "".to_string();
    myusername = "".to_string();
    match args.len() {
        // no arguments
        1 => {
            println!("Proposing menu");
            let (val0, val1) = showmenu();
            myusername = val1;
            myaction = val0;
        },
        // one argument passed / we default to user creation as read-only
        2 => {
            myusername = args[1].to_string();
            myaction = "C".to_string();
            println!("Creating User {}", myusername);
        },
        3 => {
            myusername = args[1].to_string();
            myaction = args[2].to_string();
            println!("{} User {}", myaction, myusername);
            },
        _ => {

            help();
        }
    }
    // Getting a random password
    let mypassword =  users::action::generate_random_serie();

    let config_list = match pgpass::parse_pg_pass() {
        Ok(configs) => configs,
        Err(why) => panic!("Something went wrong / please check you have a pgpass file {}", why)
    };

    let aliases = config_list.list_aliases();
    aliases
        .iter()
        .enumerate()
        .for_each(|(idx, alias)| engage(myusername.as_str(), myaction.as_str(), mypassword.as_str(), alias));

}

fn engage(myusername: &str, myaction: &str, mypassword: &str, dbalias: &str)  {
    let config_list = match parse_pg_pass() {
        Ok(configs) => configs,
        Err(why) => panic!("{}", why)
    };

    let selected = config_list.select_config(dbalias);

    let dbsc = format!("postgresql://{}@{}/{}?password={}", selected.username.as_str(), selected.hostname.as_str(), selected.dbname.as_str(), selected.password.as_str());

    println!("
    === Proceeding for User: {} & action: {} ON DB [{}] ===
    ", myusername, myaction, selected.dbname);

    match myaction.trim() {
        "C" => { println!("Create user");
            users::action::create(&dbsc, &myusername, &"read_only".to_string(), &mypassword).map_err(|err| println!("{:?}", err)).ok();
        },
        "D" => { println!("Drop");
        println!("Should we really drop this user:{}? [Y/n]", myusername);
        if asktocontinue() {
            if !myusername.is_empty() {
               users::action::drophim(&dbsc, &myusername).map_err(|err| println!("{:?}", err)).ok();
           }
        }
        },
        "P" => { users::action::change_password(&dbsc, &myusername, &mypassword).map_err(|err| println!("{:?}", err)).ok(); },
        "R" => { users::action::reviewuser(&dbsc, &myusername).map_err(|err| println!("{:?}", err)).ok(); },
        "W" => { users::action::alter(&dbsc, &myusername).map_err(|err| println!("{:?}", err)).ok(); },
        "S" => { users::action::searchuser(&dbsc, &myusername).map_err(|err| println!("{:?}", err)).ok(); }
         _ => { println!("{} is not an option!!", myaction); }
    }
}

fn showmenu() -> (String, String) {
    println!("Please select an action:s
    [C] Create user
    [D] Drop user
    [P] Change password
    [R] Review user
    [W] Alter user add Write permission
    [S] Search user");

    let whattodo = promptdata();
    println!("Review input ==");
    match whattodo.as_str().trim() {
        "C" => { println!("**Create user"); },
        "D" => { println!("**Drop"); },
        "P" => { println!("**Alter"); },
        "R" => { println!("**Review"); },
        "W" => { println!("**Alter role to WRITE"); },
        "S" => { println!("**Search user"); },
         _ => { println!("{} is not an option", whattodo); }
     }

     println!("Username?");
     let myusername = promptdata();
     (whattodo, myusername)
}

fn promptdata() -> String {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");

    return input_string.trim().to_string();
}
fn asktocontinue() -> bool {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");

    return input_string.trim().eq("Y");
}
fn help() {
    println!("usage:
rustdbadmin <string> <string>
    Check whether given string is the answer.
rustdbadmin username action {{C|D|P|R|W|S}}
    Create or drop users from all mgmt DB.");
}
