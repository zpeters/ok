//! TODO
//! - hardcode repos
//! - implement list
//! - implement go
//! - implement go *
//! - clean up list_git and list_changed
//! - rust doc - see foo examples
//! - unit tests - https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
//! - dynamic config for repos

extern crate shellexpand;

// List the git dirs found in paths
fn list_git(dirs: &[&str]) -> Option<Vec<std::path::PathBuf>> {
    use std::path::Path;

    let mut goodlist: Vec<std::path::PathBuf> = Vec::new();

    for d in dirs {
        let exp = shellexpand::tilde(d).to_string();
        let path = Path::new(&exp);
        let subdirs = path.read_dir().unwrap();
        for sub in subdirs {
            let s = sub.unwrap();
            if s.file_type().unwrap().is_dir() {
                if is_git(&s) {
                    goodlist.push(s.path())
                }
            }
        }
    }
    Some(goodlist)
}

// List only git dirs that have 'changed'
pub fn list_changed(dirs: &[&str]) -> Option<Vec<std::path::PathBuf>> {
    use std::path::Path;

    let mut goodlist: Vec<std::path::PathBuf> = Vec::new();

    for d in dirs {
        let exp = shellexpand::tilde(d).to_string();
        let path = Path::new(&exp);
        let subdirs = path.read_dir().unwrap();
        for sub in subdirs {
            let s = sub.unwrap();
            if s.file_type().unwrap().is_dir() {
                if is_git(&s) {
                    if has_changed(&s) {
                        goodlist.push(s.path())
                    }
                }
            }
        }
    }
    Some(goodlist)
}

fn has_changed(path: &std::fs::DirEntry) -> bool {
    use std::process::Command;

    let mut git = Command::new("git");
    git.arg("-C").arg(path.path()).arg("status").arg("--short");
    let stat = git.output().expect("failed to status");

    if String::from_utf8_lossy(&stat.stdout) == "" {
        false
    } else {
        true
    }
}

fn is_git(path: &std::fs::DirEntry) -> bool {
    path.path().join(".git").exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_git() {
        let paths = ["~/Projects/", "~/"];
        let result = list_git(&paths);
        assert!(result.is_some())
    }

    #[test]
    fn test_list_changed() {
        let paths = ["~/Projects/", "~/"];
        let result = list_changed(&paths);
        assert!(result.is_some())
    }
}
