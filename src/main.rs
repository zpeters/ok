extern crate clap;
extern crate ok;

use clap::{App, AppSettings, Arg, SubCommand};

pub fn main() {
    let repos = ["~/Projects/", "~/"];
    println!("My Repos: {:? }", repos);

    let matches = App::new("Ok")
        .version("0.0.1")
        .author("Zach Peters")
        .about("Ok git helper")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(SubCommand::with_name("list").about("lists all repos with 'changes'"))
        .subcommand(
            SubCommand::with_name("go")
                .about("commit and push all 'changed' repos")
                .arg(Arg::with_name("repo").multiple(true)),
        )
        .get_matches();

    if matches.is_present("list") {
        println!("Called list");
        println!("Dirs: {:#?}", ok::list_changed(&repos))
    }

    if let Some(go_matches) = matches.subcommand_matches("go") {
        if let Some(r) = go_matches.value_of("repo") {
            println!("'go' called with repo {}", r)
        } else {
            println!("Go called alone")
        };
    }
}
