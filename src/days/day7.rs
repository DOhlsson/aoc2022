/*
 * This is not my proudest creation.
 */

use std::cell::RefCell;
use std::io::BufRead;
use std::rc::Rc;

struct Directory {
    name: String,
    up: Option<DirRef>,
    contents: Vec<FileRef>,
}

struct RegularFile {
    name: String,
    size: usize,
}

trait File {
    fn name(&self) -> &String;
    fn size(&self) -> usize;
    fn is_dir(&self) -> bool;
    fn dir_contents(&self) -> Option<Vec<FileRef>>;
}

type FileRef = Rc<RefCell<dyn File>>;
type DirRef = Rc<RefCell<Directory>>;

impl Directory {
    pub fn new(name: String) -> DirRef {
        let dir = Directory {
            name,
            up: None,
            contents: Vec::new(),
        };

        return Rc::new(RefCell::new(dir));
    }

    pub fn add_dir(&mut self, dir: DirRef) {
        self.contents.push(dir);
    }

    pub fn add_file(&mut self, file: FileRef) {
        self.contents.push(file);
    }
}

impl RegularFile {
    pub fn new(name: String, size: usize) -> FileRef {
        let file = RegularFile { name, size };

        return Rc::new(RefCell::new(file));
    }
}

impl File for Directory {
    fn name(&self) -> &String {
        &self.name
    }
    fn size(&self) -> usize {
        self.contents.iter().map(|f| f.borrow_mut().size()).sum()
    }
    fn is_dir(&self) -> bool {
        true
    }
    fn dir_contents(&self) -> Option<Vec<FileRef>> {
        Some(self.contents.clone())
    }
}

impl File for RegularFile {
    fn name(&self) -> &String {
        &self.name
    }
    fn size(&self) -> usize {
        self.size
    }
    fn is_dir(&self) -> bool {
        false
    }
    fn dir_contents(&self) -> Option<Vec<FileRef>> {
        None
    }
}

pub fn day7(input: Box<dyn BufRead>) {
    println!("Day 7");

    let root = Directory::new(String::from("/"));
    let mut cwd = root.clone();
    let mut lines = input.lines().map(|l| l.unwrap()).peekable();

    while lines.peek().is_some() {
        let line = lines.next().unwrap();
        //println!("{}", line);

        let mut split = line.split(" ");
        if split.next().unwrap() != "$" {
            panic!();
        }

        match (split.next(), split.next()) {
            (Some("cd"), Some("/")) => {
                cwd = root.clone();
            }
            (Some("cd"), Some("..")) => {
                let up = cwd.borrow().up.clone().unwrap();
                cwd = up;
            }
            (Some("cd"), Some(dirname)) => {
                let dir = Directory::new(String::from(dirname));
                dir.borrow_mut().up = Some(cwd.clone());
                cwd.borrow_mut().add_dir(dir.clone());
                cwd = dir;
            }
            (Some("ls"), None) => {
                while lines.peek().is_some() && !lines.peek().unwrap().starts_with("$") {
                    let file_line = lines.next().unwrap();
                    //println!("{}", file_line);

                    let mut file_split = file_line.split(" ");
                    let size = file_split.next().unwrap().parse::<usize>();
                    if size.is_err() {
                        continue;
                    }

                    let size = size.unwrap();
                    let name = file_split.next().unwrap();

                    let file = RegularFile::new(String::from(name), size);
                    cwd.borrow_mut().add_file(file);
                }
            }
            _ => {
                panic!("Unexpected input");
            }
        }
    }

    let fs_size = root.borrow().size();
    let unused = 70_000_000 - fs_size;
    let cleanup = 30_000_000 - unused;
    println!("Total size: {}", root.borrow().size());
    println!("Clean up: {}", cleanup);

    let mut part1 = 0;
    let mut part2 = 70_000_000;

    let mut dirs: Vec<FileRef> = vec![root.clone()];
    loop {
        if dirs.len() == 0 {
            break;
        }

        let cwd = dirs.pop().unwrap();
        let name = cwd.borrow().name().clone();
        println!("Dir {}", name);
        let size = cwd.borrow().size();

        if size <= 100_000 {
            part1 += size;
        }

        if size >= cleanup && size < part2 {
            part2 = size;
            println!("Clean up dir {} with {} bytes", name, size);
        }

        for file in cwd.borrow().dir_contents().unwrap() {
            if file.borrow().is_dir() {
                dirs.push(file);
            }
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
