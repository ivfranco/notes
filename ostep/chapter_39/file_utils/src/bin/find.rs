use std::{path::PathBuf, process};

use anyhow::{bail, Context};
use file_utils::{file_name, file_type, Dir};
use nix::{dir::Type, errno::Errno, sys::stat::lstat};

fn main() {
    let args = argh::from_env();
    if let Err(e) = exec(args) {
        eprintln!("{:#}", e);
        process::exit(1);
    }
}

fn exec(args: FindArgs) -> anyhow::Result<()> {
    let dfs = DirectoryDFS::new(&args.root);
    for entry in dfs {
        let entry = entry?;
        println!("{}", entry.to_str().context("path is not valid utf8")?);
    }
    Ok(())
}

struct DirectoryDFS {
    stack: Vec<PathBuf>,
}

impl DirectoryDFS {
    fn new(root: &str) -> Self {
        Self {
            stack: vec![PathBuf::from(root)],
        }
    }
}

impl Iterator for DirectoryDFS {
    type Item = anyhow::Result<PathBuf>;

    fn next(&mut self) -> Option<Self::Item> {
        fn search(path: PathBuf, stack: &mut Vec<PathBuf>) -> anyhow::Result<PathBuf> {
            let stat = lstat(&path).context("stat() failed")?;
            if file_type(stat.st_mode) == Type::Directory {
                let dir = match Dir::open(path.to_str().context("path is not valid utf8")?) {
                    Ok(dir) => dir,
                    Err(Errno::EACCES) => {
                        eprintln!("Cannot open {:?}: no permissions", path);
                        return Ok(path);
                    }
                    Err(e) => bail!("opendir() failed: {}", e),
                };

                for entry in dir {
                    let entry = entry?;
                    let file_name = file_name(&entry)?;
                    if !(file_name == "." || file_name == "..") {
                        stack.push(path.join(file_name));
                    }
                }
            }

            Ok(path)
        }

        let path = self.stack.pop()?;
        Some(search(path, &mut self.stack))
    }
}

#[derive(argh::FromArgs)]
/// Walk a file directory.
struct FindArgs {
    #[argh(positional, default = "\".\".to_string()")]
    root: String,
}
