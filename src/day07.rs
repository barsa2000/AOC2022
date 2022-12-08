use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Parsed = Rc<RefCell<Directory>>;

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct File {
    name: String,
    size: u128,
}
impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.name, self.size)
    }
}

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    parent: Weak<RefCell<Directory>>,
    dirs: HashMap<String, Rc<RefCell<Directory>>>,
    files: Vec<File>,
}

impl Directory {
    fn new(name: &str) -> Self {
        Directory {
            name: name.to_owned(),
            parent: Weak::default(),
            dirs: HashMap::new(),
            files: Vec::new(),
        }
    }

    fn new_with_parent(name: &str, parent: &Weak<RefCell<Self>>) -> Self {
        Directory {
            name: name.to_owned(),
            parent: parent.clone(),
            dirs: HashMap::new(),
            files: Vec::new(),
        }
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    fn add_dir(&mut self, dir: Directory) {
        self.dirs
            .insert(dir.name.clone(), Rc::new(RefCell::new(dir)));
    }

    fn size(&self) -> u128 {
        let tot_files: u128 = self.files.iter().map(|f| f.size).sum();
        let tot_folders: u128 = self.dirs.values().map(|d| d.borrow().size()).sum();

        tot_files + tot_folders
    }
}

#[derive(Clone, Debug)]
struct DirectoryExplorer {
    root: Rc<RefCell<Directory>>,
    current: Weak<RefCell<Directory>>,
}

impl DirectoryExplorer {
    fn new() -> Self {
        let root = Directory::new("/");
        let root = Rc::new(RefCell::new(root));
        let current = Rc::downgrade(&root);

        Self { root, current }
    }

    fn cd(&mut self, dir: &str) {
        let next = match dir {
            ".." => self.current.upgrade().unwrap().borrow().parent.clone(),
            "/" => Rc::downgrade(&self.root),
            d => Rc::downgrade(
                self.current
                    .upgrade()
                    .unwrap()
                    .borrow_mut()
                    .dirs
                    .get_mut(d)
                    .unwrap(),
            ),
        };
        self.current = next;
    }

    fn insert_file(&mut self, file: File) {
        self.current.upgrade().unwrap().borrow_mut().add_file(file);
    }

    fn insert_dir(&mut self, dir_name: &str) {
        let dir = Directory::new_with_parent(dir_name, &self.current);
        self.current.upgrade().unwrap().borrow_mut().add_dir(dir);
    }
}

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Parsed {
    let commands = input.split("$ ").skip(1).collect_vec();
    let mut exp = DirectoryExplorer::new();

    for command in commands.iter() {
        let mut lines = command.lines();
        let comm = lines.next().unwrap();
        let mut comm = comm.split(' ');
        match comm.next().unwrap() {
            "cd" => exp.cd(comm.next().unwrap()),
            "ls" => {
                lines.for_each(|l| {
                    let mut toks = l.split(' ');
                    let dir_or_size = toks.next().unwrap();

                    if dir_or_size == "dir" {
                        let name = toks.next().unwrap().to_owned();
                        exp.insert_dir(&name);
                    } else {
                        exp.insert_file(File {
                            name: toks.next().unwrap().to_owned(),
                            size: dir_or_size.parse().unwrap(),
                        })
                    }
                });
            }
            _ => {
                unreachable!()
            }
        }
    }
    exp.root
}

fn calc_size_dup(dir: &Rc<RefCell<Directory>>, max: u128) -> u128 {
    let dir_size = dir.borrow().size();
    let dir_size = if dir_size <= max { dir_size } else { 0 };
    let sub_dirs_size: u128 = dir
        .borrow()
        .dirs
        .values()
        .map(|d| calc_size_dup(d, max))
        .sum();

    dir_size + sub_dirs_size
}

const MAX_DIR_SIZE: u128 = 100_000;

#[aoc(day7, part1)]
fn part1(input: &Parsed) -> u128 {
    calc_size_dup(input, MAX_DIR_SIZE)
}

fn find_all_dir_sizes(dir: &Rc<RefCell<Directory>>) -> Vec<u128> {
    let mut out = vec![dir.borrow().size()];
    let mut sizes = dir
        .borrow()
        .dirs
        .values()
        .flat_map(find_all_dir_sizes)
        .collect::<Vec<u128>>();

    out.append(&mut sizes);

    out
}

const DISK_SPACE: u128 = 70_000_000;
const UNUSED_NEEDED: u128 = 30_000_000;

#[aoc(day7, part2)]
fn part2(input: &Parsed) -> u128 {
    let tot_unused = DISK_SPACE.saturating_sub(input.borrow().size());
    let need_to_free = UNUSED_NEEDED.saturating_sub(tot_unused);
    let mut sizes = find_all_dir_sizes(input);
    sizes.sort_unstable();
    *sizes
        .iter()
        .filter(|s| **s >= need_to_free)
        .take(1)
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
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
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), 95437);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input())), 24933642);
    }
}
