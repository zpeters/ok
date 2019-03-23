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
                .arg(Arg::with_name("verbose").short("v").help("Verbose"))
                .arg(Arg::with_name("repo").multiple(true)),
        )
        .get_matches();

    if matches.is_present("list") {
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
                            .bright_cyan()
                            .underline()
                    );
                    println!("{}", d.results.cyan().dimmed());
                }
            }
        }
    }

    if let Some(go_matches) = matches.subcommand_matches("go") {
        if let Some(r) = go_matches.value_of("repo") {
            let changed = command::list_changed(&repos);
            match changed {
                None => println!("{}", "No changed repos".yellow()),
                Some(changed_dirs) => {
                    let dirs: Vec<GitRepo> = changed_dirs
                        .into_iter()
                        .filter(|c| c.path.to_string_lossy().contains(r))
                        .collect();
                    go_dirs(dirs, go_matches.is_present("verbose"))
                }
            }
        } else {
            let changed = command::list_changed(&repos);
            match changed {
                None => println!("{}", "No changed repos".yellow()),
                Some(dirs) => go_dirs(dirs, go_matches.is_present("verbose")),
            }
        };
    }
}

fn go_dirs(dirs: Vec<GitRepo>, verbose: bool) {
    for d in dirs {
        let pathstring = d.path.into_os_string().into_string().unwrap();
        println!("Processing {}", pathstring.bright_cyan().underline());
        command::go(&pathstring, verbose)
    }
}
