use std::{
    collections::{BTreeMap, VecDeque},
    io::{self, BufRead},
};

#[derive(Debug)]
struct Directory {
    child_dirs: BTreeMap<String, Directory>,
    children: BTreeMap<String, usize>,
}

impl Directory {
    fn new() -> Directory {
        Directory {
            child_dirs: BTreeMap::new(),
            children: BTreeMap::new(),
        }
    }

    fn lookup_path(&mut self, path: &[String]) -> &mut Directory {
        if path.is_empty() {
            self
        } else {
            if !self.child_dirs.contains_key(&path[0]) {
                self.child_dirs.insert(path[0].clone(), Directory::new());
            }
            let child_dir = self.child_dirs.get_mut(&path[0]).unwrap();
            child_dir.lookup_path(&path[1..path.len()])
        }
    }

    fn add_file(&mut self, name: &str, size: usize) {
        self.children.insert(name.to_string(), size);
    }

    fn get_total_size(&self) -> usize {
        let child_size: usize = self.children.values().sum();
        let child_dir_size: usize = self
            .child_dirs.values().map(|d| d.get_total_size())
            .sum();
        child_dir_size + child_size
    }
}

fn get_sum(dir: &Directory, limit: usize) -> usize {
    let mut dir_sum = 0;
    for child_dir in dir.child_dirs.values() {
        dir_sum += get_sum(child_dir, limit);
    }
    let this_total = dir.get_total_size();
    if this_total <= limit {
        dir_sum += this_total;
    }
    dir_sum
}

fn find_smallest(dir: &Directory, minimum_size: usize) -> Option<usize> {
    let this_size = dir.get_total_size();
    if this_size < minimum_size {
        return None;
    }
    let mut smallest = Some(this_size);
    for child_dir in dir.child_dirs.values() {
        if let Some(s) = find_smallest(child_dir, minimum_size) {
            if Some(s) < smallest {
                smallest = Some(s)
            }
        }
    }
    smallest
}

fn main() -> eyre::Result<()> {
    let mut root = Directory::new();
    let mut current_path = VecDeque::<String>::new();
    for line in io::stdin().lock().lines() {
        let line = line?;
        if line.starts_with("$ ") {
            if line == "$ ls" {
                continue;
            }
            let (cmd, args) = line[2..line.len()]
                .split_once(' ')
                .ok_or_else(|| eyre::eyre!("Failed to split command"))?;
            if cmd == "cd" {
                if args == "/" {
                    current_path.clear();
                } else if args == ".." {
                    if !current_path.is_empty() {
                        current_path.pop_back();
                    }
                } else {
                    current_path.push_back(args.to_string());
                }
            } else {
                return Err(eyre::eyre!("Unexpected cmd: {:?}", cmd));
            }
        } else {
            // Should be a directory listing.
            let (size, name) = line
                .split_once(' ')
                .ok_or_else(|| eyre::eyre!("Failed to split listing"))?;
            if size == "dir" {
                // Ignore directories
            } else {
                let size: usize = size.parse()?;
                current_path.make_contiguous();
                let cwd = root.lookup_path(current_path.as_slices().0);
                cwd.add_file(name, size);
            }
        }
    }
    let size_limit = 100000;
    let sum = get_sum(&root, size_limit);

    println!("Sum of dirs < {} is {}", size_limit, sum);

    let fs_size = 70000000;
    let free_space_needed = 30000000;
    let current_free_space = fs_size - root.get_total_size();
    let extra_free_needed = free_space_needed - current_free_space;

    println!(
        "Can free: {}",
        find_smallest(&root, extra_free_needed).unwrap()
    );

    Ok(())
}
