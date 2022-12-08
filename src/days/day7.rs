use std::{collections::BTreeMap, str::FromStr};

use crate::{solver::Solver, util::*};

pub struct Day7;

#[derive(Debug, Clone)]
pub enum Token {
    Command(Command),
    FileInfo(FileInfo),
}

impl FromStr for Token {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        if s.starts_with("$") {
            iter.next();
            let name = iter.next().unwrap();
            let path = iter.next();
            match name {
                "cd" => Ok(Command::Cd(path.unwrap().to_string())),
                "ls" => Ok(Command::Ls),
                _ => Err(format!("{name} is not a command")),
            }
            .map(|cmd| Self::Command(cmd))
        } else {
            let size = iter.next().unwrap().parse::<usize>().ok();
            let name = iter.next().unwrap().to_string();
            let dir = size.is_none();
            Ok(Self::FileInfo(FileInfo { size, name, dir }))
        }
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    Cd(String),
    Ls,
}

#[derive(Debug, Clone)]
pub struct FileInfo {
    size: Option<usize>,
    name: String,
    dir: bool,
}

impl FileInfo {
    fn new_dir(name: &str) -> FileInfo {
        FileInfo {
            size: None,
            name: name.to_string(),
            dir: true,
        }
    }
}

#[derive(Debug)]
struct FileSystem {
    files: BTreeMap<String, FileInfo>,
    cwd: Vec<String>,
}

impl FileSystem {
    fn new() -> Self {
        Self {
            files: BTreeMap::from([("/".to_string(), FileInfo::new_dir("/"))]),
            cwd: Vec::new(),
        }
    }

    fn change_dir(&mut self, path: &str) {
        match path {
            "/" => {
                self.cwd.clear();
            }
            ".." => {
                self.cwd.pop();
            }
            dir => {
                self.cwd.push(dir.to_string());
            }
        };
    }

    fn cwd_str(&self) -> String {
        format!(
            "/{}{}",
            self.cwd.join("/"),
            if self.cwd.is_empty() { "" } else { "/" }
        )
    }

    fn absolute_path(&self, dir: &str) -> String {
        format!("{}{}/", self.cwd_str(), dir)
    }

    fn add_dir(&mut self, name: &str) {
        self.files
            .insert(self.absolute_path(name), FileInfo::new_dir(name));
    }

    fn add_file(&mut self, info: &FileInfo) {
        self.files
            .insert(format!("{}{}", self.cwd_str(), info.name), info.clone());
    }

    fn calculate_sizes(&mut self) {
        let mut dirs = self
            .files
            .iter()
            .filter(|(_, info)| info.dir)
            .map(|(path, info)| (path.clone(), info.clone()))
            .collect::<Vec<_>>();
        dirs.sort_unstable_by(|(a, _), (b, _)| b.cmp(a));
        dirs.iter().for_each(|(path, _)| {
            let (from, mut to) = (path.clone(), path.clone());
            to.replace_range(
                to.len() - 1..,
                char::from_u32(to.chars().last().unwrap() as u32 + 1)
                    .unwrap()
                    .to_string()
                    .as_str(),
            );
            let total_size: usize = self
                .files
                .range(from..to)
                .filter(|(_, info)| !info.dir)
                .map(|(_, info)| info.size.unwrap())
                .sum();
            let dir = self.files.get_mut(path).unwrap();
            dir.size = Some(total_size);
        });
    }
}

impl<'a> Solver<'a> for Day7 {
    type Parsed = Vec<Token>;
    type Output = usize;

    fn parse(input: &'a str) -> Self::Parsed {
        input
            .lines()
            .flat_map(|line| Token::from_str(line))
            .collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        let mut file_system = data
            .iter()
            .fold(FileSystem::new(), |mut file_system, token| {
                match token {
                    Token::Command(cmd) => {
                        if let Command::Cd(dir) = cmd {
                            file_system.change_dir(dir);
                        }
                    }
                    Token::FileInfo(info) => {
                        if info.dir {
                            file_system.add_dir(&info.name)
                        } else {
                            file_system.add_file(info)
                        }
                    }
                };
                file_system
            });
        file_system.calculate_sizes();
        file_system
            .files
            .values()
            .filter(|info| info.dir && info.size.unwrap() <= 100000)
            .map(|info| info.size.unwrap())
            .sum()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        let mut file_system = data
            .iter()
            .fold(FileSystem::new(), |mut file_system, token| {
                match token {
                    Token::Command(cmd) => {
                        if let Command::Cd(dir) = cmd {
                            file_system.change_dir(dir);
                        }
                    }
                    Token::FileInfo(info) => {
                        if info.dir {
                            file_system.add_dir(&info.name)
                        } else {
                            file_system.add_file(info)
                        }
                    }
                };
                file_system
            });
        file_system.calculate_sizes();
        let total_available = 70_000_000;
        let total_needed = 30_000_000;
        let unused = total_available - file_system.files.get("/").unwrap().size.unwrap();
        let required = total_needed - unused;
        file_system
            .files
            .values()
            .filter(|info| info.dir && info.size.unwrap() >= required)
            .map(|info| info.size.unwrap())
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_system() {
        let mut fs = FileSystem::new();
        assert_eq!(fs.cwd_str(), "/");
        assert!(fs.files.contains_key("/"));
        fs.add_dir("a");
        assert!(fs.files.contains_key("/a/"));
        assert_eq!(fs.cwd_str(), "/");
        fs.change_dir("a");
        assert_eq!(fs.cwd_str(), "/a/");
        fs.change_dir("..");
        assert_eq!(fs.cwd_str(), "/");
    }

    #[test]
    fn d7p1() {
        assert_eq!(
            Day7::part1(Day7::parse(
                "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
            )),
            95437
        );
    }

    #[test]
    fn d7p2() {
        assert_eq!(
            Day7::part2(Day7::parse(
                "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
            )),
            24933642
        );
    }
}
