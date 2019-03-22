//! TODO
//! - add verbose switch to turn on and off output (see commented exampel at the bottom using duct crate)
//! - refactor list_changed
//! - refactor main ui
//! - add more tests
//! - better doc
//! - test coverage
//! - hardcode repos - fix list

/// Higher level commands used by the UI
pub mod command {
    extern crate colored;
    extern crate shellexpand;

    use crate::git::{add, changes, commit, is_git, pull, push};
    use colored::*;

    /// A `GitRepo` stuct to return back to `list`
    ///
    /// this consists of the canonical path and the results
    /// returned from `git status --short`
    #[derive(Debug)]
    pub struct GitRepo {
        pub path: std::path::PathBuf,
        pub results: String,
    }

    /// "Go" on a git repo
    ///
    /// git pull, add all, commit and push
    pub fn go(path: &str, verbose: bool) {
        let pull_resp = pull(path, verbose);
        check_status("Pull", pull_resp);
        let add_resp = add(path, verbose);
        check_status("Add", add_resp);
        let commit_resp = commit(path, verbose);
        check_status("Commit", commit_resp);
        let push_resp = push(path, verbose);
        check_status("Push", push_resp);
    }

    fn check_status(msgtype: &str, status: bool) {
        if status {
            println!("\t{}: {}", msgtype.bright_cyan(), "Success".green())
        } else {
            println!("\t{}: {}", msgtype.bright_cyan(), "Failure".red());
            panic!(format!("Can't continue {} command failed", msgtype))
        }
    }

    /// List only git dirs that have 'changed'
    pub fn list_changed(dirs: &[&str]) -> Option<Vec<GitRepo>> {
        use std::path::Path;

        let mut goodlist: Vec<GitRepo> = Vec::new();

        for d in dirs {
            let exp = shellexpand::tilde(d).to_string();
            let path = Path::new(&exp);
            let sublist = path.read_dir();
            if let Ok(subdirs) = sublist {
                for sub in subdirs {
                    let s = sub.unwrap();
                    if s.file_type().unwrap().is_dir() {
                        if is_git(&s.path().to_str().unwrap()) {
                            if let Some(c) = changes(&s.path().to_str().unwrap()) {
                                goodlist.push(GitRepo {
                                    path: s.path(),
                                    results: c,
                                });
                            }
                        }
                    }
                }
            };
        }
        if goodlist.len() == 0 {
            None
        } else {
            Some(goodlist)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_list_changed() {
            let paths = ["~/Projects/", "~/"];
            let result = list_changed(&paths);
            match result {
                None => assert!(true),
                _ => assert!(true),
            }
        }

        #[test]
        fn test_none_list_changed() {
            let paths = ["~/FAKE"];
            let result = list_changed(&paths);
            assert!(result.is_none())
        }
    }
}

/// Lower level git commands
pub mod git {

    /// Commit everything in `filpath` git repo
    pub fn commit(filepath: &str, verbose: bool) -> bool {
        use std::path::Path;
        use std::process::{Command, Stdio};

        let mut git = Command::new("git");
        let p = Path::new(filepath);

        if verbose {
            git.stdout(Stdio::inherit()).stderr(Stdio::inherit());
        } else {
            git.stdout(Stdio::null()).stderr(Stdio::null());
        }

        let output = git
            .arg("-C")
            .arg(p)
            .arg("commit")
            .arg("-am")
            .arg("'autocommit by ok'")
            .status()
            .expect("commit should succeed");
        output.success()
    }

    /// Add everything in `filpath` git repo
    pub fn add(filepath: &str, verbose: bool) -> bool {
        use std::path::Path;
        use std::process::{Command, Stdio};

        let mut git = Command::new("git");
        let p = Path::new(filepath);

        if verbose {
            git.stdout(Stdio::inherit()).stderr(Stdio::inherit());
        } else {
            git.stdout(Stdio::null()).stderr(Stdio::null());
        }

        let output = git
            .arg("-C")
            .arg(p)
            .arg("add")
            .arg(".")
            .status()
            .expect("add should succeed");
        output.success()
    }

    /// Push from `filepath` git repo
    pub fn push(filepath: &str, verbose: bool) -> bool {
        use std::path::Path;
        use std::process::{Command, Stdio};

        let mut git = Command::new("git");
        let p = Path::new(filepath);

        if verbose {
            git.stdout(Stdio::inherit()).stderr(Stdio::inherit());
        } else {
            git.stdout(Stdio::null()).stderr(Stdio::null());
        }

        let output = git
            .arg("-C")
            .arg(p)
            .arg("push")
            .status()
            .expect("push should succeed");
        output.success()
    }

    /// Pull from `filepath` git repo
    pub fn pull(filepath: &str, verbose: bool) -> bool {
        use std::path::Path;
        use std::process::{Command, Stdio};

        let mut git = Command::new("git");
        let p = Path::new(filepath);

        if verbose {
            git.stdout(Stdio::inherit()).stderr(Stdio::inherit());
        } else {
            git.stdout(Stdio::null()).stderr(Stdio::null());
        }

        let output = git
            .arg("-C")
            .arg(p)
            .arg("pull")
            .status()
            .expect("pull should succeed");
        output.success()
    }

    /// Checks `git status` of the `filepath` repo and returns `true` if there is any output
    pub fn changes(filepath: &str) -> Option<String> {
        use std::path::Path;
        use std::process::Command;

        let mut git = Command::new("git");
        let p = Path::new(filepath);
        git.arg("-C").arg(p).arg("status").arg("--short");
        let stat = git.output().expect("failed to status");
        let out = String::from_utf8_lossy(&stat.stdout);

        if out == "" {
            None
        } else {
            Some(out.to_string())
        }
    }

    /// Checks if `filepath` is a git dir or not (by checking for a `.git` subfolder)
    pub fn is_git(filepath: &str) -> bool {
        use std::path::Path;
        let p = Path::new(filepath);
        p.join(".git").exists()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_git_push_success() {
            assert!(push("."));
        }

        #[test]
        fn test_git_push_fail() {
            assert_eq!(push("FAKE"), false);
        }

        #[test]
        fn test_git_pull_success() {
            assert!(pull("."));
        }

        #[test]
        fn test_git_pull_fail() {
            assert_eq!(pull("FAKE"), false);
        }

        #[test]
        fn test_is_git() {
            assert!(is_git("."));
        }

        #[test]
        fn test_is_not_git() {
            assert_eq!(is_git("/tmp"), false);
        }

        #[test]
        fn test_changes() {
            changes(".");
        }

        #[test]
        fn test_no_changes() {
            let resp = changes("/tmp");
            match resp {
                Some(_) => assert!(false, "should not have changes"),
                None => assert!(true),
            }
        }

    }
}

// extern crate duct;
// use duct::*;

// fn main() {
//     let r1 = quiet("8.8.8.8");
//     println!("Quiet Good: {}", r1);
//     let r2 = quiet("xxxxx");
//     println!("Quiet Bad: {}", r2);

//     let r3 = norm("8.8.8.8");
//     println!("Norm Good: {}", r3);
//     let r4 = norm("xxxxx");
//     println!("Norm Bad: {}", r4)
// }

// fn quiet(host: &str) -> bool {
//     let qp = cmd!("ping", "-c", "3", host).stderr_to_stdout().read();
//     match qp {
//         Ok(_) => return true,
//         Err(_) => return false,
//     }
// }

// fn norm(host: &str) -> bool {
//     let args = &["-c", "3", host];
//     let p = cmd("ping", args).run();
//     match p {
//         Ok(_) => return true,
//         Err(_) => return false,
//     }
// }
