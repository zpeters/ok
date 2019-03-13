extern crate clap;

use clap::{App, SubCommand, AppSettings, Arg};

fn main() {
    let matches = App::new("Ok")
        .version("0.0.1")
        .author("Zach Peters")
        .about("Ok git helper")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(SubCommand::with_name("list")
                    .about("lists all repos with 'changes'"))
        .subcommand(SubCommand::with_name("go")
                    .about("commit and push all 'changed' repos")
                    .arg(Arg::with_name("repo")
                         .multiple(true)))
        .get_matches();

    if matches.is_present("list") {
        println!("Called list")
    }

    if let Some(go_matches) = matches.subcommand_matches("go") {
        if let Some(r) = go_matches.value_of("repo") {
            println!("'go' called with repo {}", r)
        } else {
            println!("Go called alone")
        };
    }
}
