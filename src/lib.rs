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

    fn check_status(msg: &str, status: bool) {
        if status {
            print_success(msg);
        } else {
            print_failure(msg);
            panic!("Can't continue {} command failed", msg)
        }
    }

    fn print_success(msg: &str) {
        println!("{}: {}", msg.bright_cyan(), "Success".green())
    }

    fn print_failure(msg: &str) {
        println!("{}: {}", msg.bright_cyan(), "Failure".red());
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
                    if s.file_type().unwrap().is_dir() && is_git(s.path().to_str().unwrap()) {
                        if let Some(c) = changes(s.path().to_str().unwrap()) {
                            goodlist.push(GitRepo {
                                path: s.path(),
                                results: c,
                            });
                        }
                    }
                }
            };
        }
        if goodlist.is_empty() {
            None
        } else {
            Some(goodlist)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        // TODO figure out how to actually test this
        //#[test]
        //fn test_list_changed() {
        //    let paths = ["~/Projects/", "~/"];
        //    let result = list_changed(&paths);
        //    assert!(result.is_some());
        //}

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

    fn git_command(git_args: Vec<&str>, filepath: &str, verbose: bool) -> bool {
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
            .args(git_args)
            .status()
            .expect("command should succeed");
        output.success()
    }

    /// Commit everything in `filpath` git repo
    pub fn commit(filepath: &str, verbose: bool) -> bool {
        let args = vec!["commit", "-am", "'autocommit by ok'"];
        git_command(args, filepath, verbose)
    }

    /// Add everything in `filpath` git repo
    pub fn add(filepath: &str, verbose: bool) -> bool {
        let args = vec!["add", "."];
        git_command(args, filepath, verbose)
    }

    /// Push from `filepath` git repo
    pub fn push(filepath: &str, verbose: bool) -> bool {
        let args = vec!["push"];
        git_command(args, filepath, verbose)
    }

    /// Pull from `filepath` git repo
    pub fn pull(filepath: &str, verbose: bool) -> bool {
        let args = vec!["pull"];
        git_command(args, filepath, verbose)
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
        fn test_git_push_fail() {
            assert!(!push("FAKE", false));
        }

        #[test]
        fn test_git_pull_fail() {
            assert!(!pull("FAKE", false));
        }

        #[test]
        fn test_is_git() {
            assert!(is_git("."));
        }

        #[test]
        fn test_is_not_git() {
            assert!(!is_git("/tmp"));
        }

        #[test]
        fn test_changes() {
            changes(".");
        }

        #[test]
        fn test_no_changes() {
            let resp = changes("/tmp");
            assert!(resp.is_none());
        }
    }
}
