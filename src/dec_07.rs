use std::{collections::HashMap, vec};

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
    File(u32, String),
}

// ---------------------------------------------

#[derive(Debug)]
struct Tree {
    dirs: Vec<Dir>,
    children: HashMap<usize, HashMap<String, usize>>,
}

#[derive(Debug)]
struct Dir {
    index: usize,
    parent: Option<usize>,
    size_of_contained_files: u32,
}

impl Tree {
    fn get(&self, index: usize) -> &Dir {
        &self.dirs[index]
    }
}

impl Dir {
    fn get_size(&self, tree: &Tree) -> u32 {
        let mut sum = self.size_of_contained_files;
        if let Some(children) = tree.children.get(&self.index) {
            for (_, child_index) in children {
                sum += tree.get(*child_index).get_size(tree);
            }
        }
        sum
    }
}

impl Tree {
    fn get_size(&self) -> u32 {
        self.get(0).get_size(self)
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
                let size = parts.next().unwrap().parse::<u32>().unwrap();
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

    let mut tree = Tree {
        dirs: vec![Dir {
            index: 0,
            parent: None,
            size_of_contained_files: 0,
        }],
        children: HashMap::new(),
    };

    let mut current: usize = 0;

    for command in commands {
        match command {
            Command::Ls(entries) => {
                for entry in entries {
                    match entry {
                        Entry::Dir(dir_name) => {
                            let index = tree.dirs.len();
                            let dir = Dir {
                                index,
                                parent: Some(current),
                                size_of_contained_files: 0,
                            };
                            tree.dirs.push(dir);

                            tree.children
                                .entry(current)
                                .or_insert_with(|| HashMap::new())
                                .insert(dir_name, index);
                        }
                        Entry::File(size, _) => {
                            tree.dirs.get_mut(current).unwrap().size_of_contained_files += size;
                        }
                    }
                }
            }
            Command::CdRoot => current = 0,
            Command::CdOut => current = tree.get(current).parent.unwrap(),
            Command::CdIn(name) => {
                let child_index = tree.children.get(&current).unwrap().get(&name).unwrap();

                current = *child_index;
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
            .sum::<u32>()
            .to_string()
    }

    fn solve_two(&self, _input: &str, lines: &Vec<&str>) -> String {
        let total_memory = 70000000;
        let meory_needed = 30000000;

        let tree = get_tree(lines);

        let currently_free = total_memory - tree.get_size();
        let additinally_needed = meory_needed - currently_free;

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
