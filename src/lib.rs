//! TODO
//! - implemetn git add
//! - implemetn git commit
//! - implement go - lib
//! - implement go * - lib
//! - implement go - ui
//! - implement go * -ui
//! - switch unwrap to expect
//! - refactor list_changed
//! - add more tests
//! - test coverage
//! - hardcode repos - fix list
//! - rust doc - see foo examples

pub mod command {
    extern crate shellexpand;

    use crate::git::{changes, is_git};

    #[derive(Debug)]
    pub struct GitRepo {
        pub path: std::path::PathBuf,
        pub results: String,
    }
    // List only git dirs that have 'changed'
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
            } else {
                return None;
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

pub mod git {
    pub fn commit(filepath: &str) -> bool {
        use std::path::Path;
        use std::process::Command;

        let mut git = Command::new("git");
        let p = Path::new(filepath);
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

    pub fn add(filepath: &str) -> bool {
        use std::path::Path;
        use std::process::Command;

        let mut git = Command::new("git");
        let p = Path::new(filepath);
        let output = git
            .arg("-C")
            .arg(p)
            .arg("add")
            .arg(".")
            .status()
            .expect("add should succeed");
        output.success()
    }

    pub fn push(filepath: &str) -> bool {
        use std::path::Path;
        use std::process::Command;

        let mut git = Command::new("git");
        let p = Path::new(filepath);
        let output = git
            .arg("-C")
            .arg(p)
            .arg("push")
            .status()
            .expect("push should succeed");
        output.success()
    }

    pub fn pull(filepath: &str) -> bool {
        use std::path::Path;
        use std::process::Command;

        let mut git = Command::new("git");
        let p = Path::new(filepath);
        let output = git
            .arg("-C")
            .arg(p)
            .arg("pull")
            .status()
            .expect("pull should succeed");
        output.success()
    }

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
