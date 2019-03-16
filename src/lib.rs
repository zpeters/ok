//! TODO
//! - create a directory struct
//! - hardcode repos - fix list
//! - implement list - ui
//! - implement go - ui
//! - implement go * -ui
//! - implement go - lib
//! - implement go * - lib
//! - refactor list_changed
//! - rust doc - see foo examples

extern crate shellexpand;

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
                    if is_git(&s) {
                        if let Some(c) = changes(&s) {
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

fn changes(path: &std::fs::DirEntry) -> Option<String> {
    use std::process::Command;

    let mut git = Command::new("git");
    git.arg("-C").arg(path.path()).arg("status").arg("--short");
    let stat = git.output().expect("failed to status");
    let out = String::from_utf8_lossy(&stat.stdout);

    if out == "" {
        None
    } else {
        Some(out.to_string())
    }
}

fn is_git(path: &std::fs::DirEntry) -> bool {
    path.path().join(".git").exists()
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
