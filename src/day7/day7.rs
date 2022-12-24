// Tree problem!
// Start with single root node as uninitialized
// Have a pointer pointing to the current dir
// If "ls" called on node:
// if uninit: Init + add files + make subdirs
// if init: Do nothing
// If "cd" called on node:
// Change pointer to root
// If "cd .."  called on node:
// Change pointer to parent dir
// If "cd /" called on node:
// Change pointer to root
// Traverse through tree recursively, adding all dirs to list. Then sort and find number which <100K.

// Dir struct:
// Vec of subdirs
// Reference to parent
// File size count

use std::cell::RefCell;
use std::rc::Rc;

struct Dir {
    init: bool,
    subdirs: Vec<Rc<RefCell<Dir>>>,
    parent: Option<Rc<RefCell<Dir>>>,
    size: u32,
    name: String,
}

impl Dir {
    fn new(name: String, parent: Option<Rc<RefCell<Dir>>>) -> Self {
        Dir {
            init: false,
            subdirs: Vec::new(),
            parent,
            size: 0,
            name,
        }
    }

    fn root() -> Self {
        Self::new('/'.to_string(), None)
    }
}

pub fn day7_p1(input: &str) -> u32 {
    let dir_sizes = day7(input);
    dir_sizes.iter().filter(|size| **size < 100_000).sum()
}

pub fn day7_p2(input: &str) -> u32 {
    let mut dir_sizes = day7(input);
    dir_sizes.sort();

    let available_space = 70_000_000 - dir_sizes.last().unwrap();

    *dir_sizes
        .iter()
        .find(|dir_size| available_space + **dir_size > 30_000_000)
        .unwrap()
}

fn day7(input: &str) -> Vec<u32> {
    // Make iterator over lines
    let mut input = input.lines().peekable();

    // Make tree data structure to describe file system
    let root = Rc::new(RefCell::new(Dir::root()));
    let mut wd = root.clone();

    while input.peek().is_some() {
        // Attempt to read next instruction, break if at the end
        let line = input.next().unwrap();

        if line == "$ cd /" {
            // Return to root
            wd = root.clone();
        } else if line == "$ cd .." {
            // Move up a level
            let parent = wd.borrow().parent.as_ref().unwrap().clone();
            wd = parent.clone();
        } else if line == "$ ls" {
            if true {
                // Init directory and read all files
                while input.peek().is_some() && !input.peek().unwrap().starts_with('$') {
                    let line = input.next().unwrap();

                    if line.starts_with("dir") {
                        // Found new directory
                        let name = line.split(' ').nth(1).unwrap().to_string();

                        // Confirm we haven't seen directory yet, then add
                        if !wd
                            .borrow()
                            .subdirs
                            .iter()
                            .any(|subdir| subdir.borrow().name == name)
                        {
                            let new_dir = RefCell::new(Dir::new(name, Some(wd.clone())));
                            wd.borrow_mut().subdirs.push(Rc::new(new_dir));
                        }
                    } else {
                        // Found a file, add to size
                        wd.borrow_mut().size +=
                            line.split(' ').next().unwrap().parse::<u32>().unwrap();
                    }
                }

                wd.borrow_mut().init = true;
            }
        } else {
            // Has to be a "cd x" command, get directory to cd into
            let name = line.split(' ').nth(2).unwrap().to_string();

            let dest = wd
                .borrow()
                .subdirs
                .iter()
                .find(|subdir| subdir.borrow().name == name)
                .unwrap()
                .clone();
            wd = dest;
        }
    }

    fn print_dir(dir: &Dir, indent: String) {
        let parent = dir.parent.as_ref();
        if parent.is_some() {
            println!(
                "{indent}Dir {} of size {} with parent {}.",
                dir.name,
                dir.size,
                parent.unwrap().borrow().name
            );
        } else {
            println!(
                "{indent}Dir {} of size {} with no parent.",
                dir.name, dir.size
            );
        }

        for subdir in &dir.subdirs {
            print_dir(&subdir.borrow(), indent.to_string() + "  ");
        }
    }

    print_dir(&root.borrow(), "".to_string());

    fn add_dir(dir: &Dir, dir_sizes: &mut Vec<u32>) -> u32 {
        let mut size = dir.size;

        for subdir in &dir.subdirs {
            size += add_dir(&subdir.borrow(), dir_sizes);
        }

        dir_sizes.push(size);
        size
    }

    let mut dir_sizes: Vec<u32> = Vec::new();
    add_dir(&root.borrow(), &mut dir_sizes);
    dir_sizes
}
