use std::cell::RefCell;
use std::io::BufRead;
use std::rc::Rc;

struct Directory {
    name: String,
    up: Option<DirRef>,
    contents: Vec<FileRef>,
}

struct DirIter<'a>(&'a Directory);

struct RegularFile {
    name: String,
    size: usize,
}

trait File {
    fn name(&self) -> &String;
    fn size(&self) -> usize;
    fn is_dir(&self) -> bool;
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

    pub fn iter(&self) -> DirIter {
        DirIter(self)
    }
}

impl RegularFile {
    pub fn new(name: String, size: usize) -> FileRef {
        let file = RegularFile {
            name,
            size,
        };

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
}

impl<'a> Iterator for DirIter<'a> {
    type Item = &'a Directory;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

pub fn day7(input: Box<dyn BufRead>) {
    println!("Day 7");

    let root = Directory::new(String::from("/"));
    let mut cwd = root.clone();
    let mut lines = input.lines().map(|l| l.unwrap()).peekable();

    let mut part1 = 0;
    let mut part2 = 0;

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
            },
            (Some("cd"), Some(dirname)) => {
                let dir = Directory::new(String::from(dirname));
                dir.borrow_mut().up = Some(cwd.clone());
                cwd.borrow_mut().add_dir(dir.clone());
                cwd = dir;
            },
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

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Total size: {}", root.borrow().size());

    // I intended to make an iterator, but got too tired
    // let part1: usize = root.borrow().iter().filter(|f| f.is_dir()).map(|f| f.size()).filter(|f| *f <= 100_000).sum();
    // println!("Part 1: {}", part1);

    let test = vec![1, 11, 2, 12, 3, 13, 4];
    let collect: Vec<i32> = test.into_iter().filter(|i| *i < 10).collect();
    println!("Test {:?}", collect);

    
}
