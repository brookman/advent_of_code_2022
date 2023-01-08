use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    vec,
};

use crate::common::Solution;

pub struct Dec07 {}

#[derive(Debug)]
enum Command {
    Ls(Vec<Entry>),
    CdRoot,
    CdOut,
    CdIn(String),
}

#[derive(Debug)]
enum Entry {
    Dir(String),
    File(usize, String),
}

// ---------------------------------------------

type DirIdex = usize;
type FileIndex = usize;

#[derive(Debug)]
struct Tree {
    dirs: Vec<Dir>,
    files: Vec<File>,
    root: DirIdex,
    children: HashMap<DirIdex, HashSet<ChildIndex>>,
    children_by_name: HashMap<DirIdex, HashMap<String, ChildIndex>>,
}

#[derive(Debug)]
struct Dir {
    index: DirIdex,
    parent: Option<DirIdex>,
    name: String,
}

#[derive(Debug)]
struct File {
    index: FileIndex,
    parent: DirIdex,
    name: String,
    size: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum ChildIndex {
    Dir(DirIdex),
    File(FileIndex),
}

impl Tree {
    fn get(&self, index: &ChildIndex) -> Option<&dyn Sizable> {
        match index {
            ChildIndex::Dir(dir_index) => self.get_dir(*dir_index).map(|x| x as &dyn Sizable),
            ChildIndex::File(file_inxed) => self.get_file(*file_inxed).map(|x| x as &dyn Sizable),
        }
    }

    fn get_dir(&self, index: DirIdex) -> Option<&Dir> {
        if index >= self.dirs.len() {
            return None;
        }
        Some(&self.dirs[index])
    }

    fn get_file(&self, index: FileIndex) -> Option<&File> {
        if index >= self.files.len() {
            return None;
        }
        Some(&self.files[index])
    }

    fn get_children_names(&self, index: DirIdex) -> HashSet<String> {
        if let Some(children) = self.children.get(&index) {
            return children
                .iter()
                .flat_map(|i| match i {
                    ChildIndex::Dir(dir_index) => {
                        self.get_dir(*dir_index).map(|dir| dir.name.clone())
                    }
                    ChildIndex::File(file_index) => {
                        self.get_file(*file_index).map(|file| file.name.clone())
                    }
                })
                .collect();
        }
        HashSet::new()
    }
}

trait Sizable {
    fn get_size(&self, tree: &Tree) -> usize;
}

impl Sizable for File {
    fn get_size(&self, _tree: &Tree) -> usize {
        self.size
    }
}

impl Sizable for Dir {
    fn get_size(&self, tree: &Tree) -> usize {
        let mut sum = 0;
        for child_index in tree.children.get(&self.index).unwrap() {
            sum += tree.get(child_index).unwrap().get_size(tree);
        }
        sum
    }
}

impl Sizable for Tree {
    fn get_size(&self, tree: &Tree) -> usize {
        self.get(&ChildIndex::Dir(tree.root))
            .unwrap()
            .get_size(tree)
    }
}

fn get_tree(lines: &Vec<&str>) -> Tree {
    let mut commands: Vec<Command> = vec![];

    for line in lines {
        if line.starts_with("$ ") {
            let command_str = &line[2..];
            if command_str.starts_with("ls") {
                commands.push(Command::Ls(vec![]));
            } else if command_str.starts_with("cd /") {
                commands.push(Command::CdRoot);
            } else if command_str.starts_with("cd ..") {
                commands.push(Command::CdOut);
            } else {
                commands.push(Command::CdIn(command_str[3..].to_string()));
            }
        } else if line.starts_with("dir ") {
            match commands.last_mut().unwrap() {
                Command::Ls(ls) => ls.push(Entry::Dir(line[4..].to_string())),
                Command::CdRoot => {}
                Command::CdOut => {}
                Command::CdIn(_) => {}
            }
        } else {
            let entry = if line.starts_with("dir ") {
                Entry::Dir(line[4..].to_string())
            } else {
                let mut parts = line.split(" ");
                let size = parts.next().unwrap().parse::<usize>().unwrap();
                let file_name = parts.next().unwrap().to_string();
                Entry::File(size, file_name)
            };

            match commands.last_mut().unwrap() {
                Command::Ls(ls) => ls.push(entry),
                Command::CdRoot => {}
                Command::CdOut => {}
                Command::CdIn(_) => {}
            }
        }
    }

    // println!("commands {:?}", commands);

    let mut tree = Tree {
        dirs: vec![Dir {
            index: 0,
            parent: None,
            name: "/".to_string(),
        }],
        files: vec![],
        root: 0,
        children: HashMap::new(),
        children_by_name: HashMap::new(),
    };

    let mut current_dir_index: DirIdex = 0;

    for command in commands {
        let children_names = tree.get_children_names(current_dir_index);
        match command {
            Command::Ls(entries) => {
                for entry in entries {
                    match entry {
                        Entry::Dir(dir_name) => {
                            if !children_names.contains(&dir_name) {
                                let index = tree.dirs.len();
                                let dir = Dir {
                                    index,
                                    parent: Some(current_dir_index),
                                    name: dir_name.clone(),
                                };
                                tree.dirs.push(dir);

                                tree.children
                                    .entry(current_dir_index)
                                    .or_insert_with(|| HashSet::new())
                                    .insert(ChildIndex::Dir(index));
                                tree.children_by_name
                                    .entry(current_dir_index)
                                    .or_insert_with(|| HashMap::new())
                                    .insert(dir_name, ChildIndex::Dir(index));
                            }
                        }
                        Entry::File(size, file_name) => {
                            if !children_names.contains(&file_name) {
                                let index = tree.files.len();
                                let file = File {
                                    index,
                                    parent: current_dir_index,
                                    name: file_name.clone(),
                                    size,
                                };
                                tree.files.push(file);

                                tree.children
                                    .entry(current_dir_index)
                                    .or_insert_with(|| HashSet::new())
                                    .insert(ChildIndex::File(index));
                                tree.children_by_name
                                    .entry(current_dir_index)
                                    .or_insert_with(|| HashMap::new())
                                    .insert(file_name, ChildIndex::File(index));
                            }
                        }
                    }
                }
            }
            Command::CdRoot => current_dir_index = 0,
            Command::CdOut => {
                current_dir_index = tree.get_dir(current_dir_index).unwrap().parent.unwrap()
            }
            Command::CdIn(name) => {
                let child_index = tree
                    .children_by_name
                    .get(&current_dir_index)
                    .unwrap()
                    .get(&name)
                    .unwrap();

                match child_index {
                    ChildIndex::Dir(dir_index) => current_dir_index = *dir_index,
                    ChildIndex::File(_) => panic!("can't cd into file"),
                }
            }
        }
    }
    tree
}

impl Solution for Dec07 {
    fn solve_one(&self, _input: &str, lines: &Vec<&str>) -> String {
        let tree = get_tree(lines);

        tree.dirs
            .iter()
            .map(|d| d.get_size(&tree))
            .filter(|s| *s <= 100000)
            .sum::<usize>()
            .to_string()
    }

    fn solve_two(&self, _input: &str, lines: &Vec<&str>) -> String {
        let total_memory = 70000000;
        let meory_needed = 30000000;

        let tree = get_tree(lines);
        let tree_size = tree.get_size(&tree);
        let currently_free = total_memory - tree_size;
        let additinally_needed = meory_needed - currently_free;

        println!(
            "tree_size {:?}, currently_free {:?}, additinally_needed {:?}",
            tree_size, currently_free, additinally_needed
        );

        tree.dirs
            .iter()
            .map(|d| d.get_size(&tree))
            .filter(|s| *s >= additinally_needed)
            .min()
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{solve_one, solve_two};

    #[test]
    fn solution_one() {
        let input = "$ cd /
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
7214296 k";
        let expected = "95437";
        assert_eq!(solve_one(&Dec07 {}, input), expected);
    }

    #[test]
    fn solution_two() {
        let input = "$ cd /
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
7214296 k";
        let expected = "24933642";
        assert_eq!(solve_two(&Dec07 {}, input), expected);
    }
}
