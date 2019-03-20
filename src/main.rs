extern crate clap;
extern crate colored;

extern crate ok;

use ok::command;
use ok::command::GitRepo;

use clap::{App, AppSettings, Arg, SubCommand};
use colored::*;

pub fn main() {
    let repos = ["~/Projects/", "~/"];

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
        println!("[Repos with changes]");
        let changed = command::list_changed(&repos);
        match changed {
            None => println!("{}", "No changed repos".yellow()),
            Some(dirs) => {
                for d in dirs {
                    println!(
                        "{}",
                        d.path
                            .into_os_string()
                            .into_string()
                            .unwrap()
                            .blue()
                            .underline()
                    );
                    println!("{}", d.results.blue().dimmed());
                }
            }
        }
    }

    if let Some(go_matches) = matches.subcommand_matches("go") {
        if let Some(r) = go_matches.value_of("repo") {
            let changed = command::list_changed(&repos).unwrap();
            let dirs: Vec<GitRepo> = changed
                .into_iter()
                .filter(|c| c.path.to_string_lossy().contains(r))
                .collect();
            for d in dirs {
                let pathstring = d.path.into_os_string().into_string().unwrap();
                println!("Processing {}", pathstring.blue().underline());
                command::go(&pathstring)
            }
        } else {
            let changed = command::list_changed(&repos);
            match changed {
                None => println!("{}", "No changed repos".yellow()),
                Some(dirs) => {
                    for d in dirs {
                        let pathstring = d.path.into_os_string().into_string().unwrap();
                        println!("Processing {}", pathstring.blue().underline());
                        command::go(&pathstring)
                    }
                }
            }
        };
    }
}
