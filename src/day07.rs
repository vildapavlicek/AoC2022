use std::collections::HashMap;
use std::convert::Infallible;
use std::iter::Peekable;
use std::slice::Iter;

type FileName = String;

#[derive(Debug)]
pub struct FilePath<'i>(Vec<&'i str>);

impl<'i> FilePath<'i> {
    pub fn iter(&self) -> std::slice::Iter<'_, &str> {
        self.0.iter()
    }

    pub fn peek(&self) -> Peekable<Iter<'_, &str>> {
        self.0.iter().peekable()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn pop(&mut self) {
        self.0.pop();
    }

    pub fn push(&mut self, item: &'i str) {
        self.0.push(item);
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct FileSystem(HashMap<FileName, FileDescriptor>);

impl FileSystem {
    pub fn add_folder(&mut self, path: &FilePath, name: &str) {
        if path.is_empty() {
            return;
        }

        let mut iter = path.iter();
        let root = iter.next().unwrap().to_string();

        self.0
            .entry(root.clone())
            .or_insert(FileDescriptor::Folder {
                content: HashMap::new(),
                name: root.to_string(),
                size: 0,
                parent: None,
            })
            .insert_folder(&mut iter, root.as_str(), name);
    }

    pub fn add_file(&mut self, path: &FilePath, name: &str, size: u32) {
        if path.is_empty() {
            return;
        }

        let mut peekable_iter = path.peek();
        let root = peekable_iter.next().unwrap();

        match self.0.get_mut(&root.to_string()) {
            Some(folder) => folder.insert_file(&mut peekable_iter, root, name, size),
            None => unreachable!(),
        }
    }

    pub fn sum_folder_sizes_for(&self, counter: &mut u32) {
        self.0
            .get(&"/".to_string())
            .expect("root not found!")
            .sum_folder_sizes(counter)
    }

    pub fn used_space(&self) -> u32 {
        if let FileDescriptor::Folder { size, .. } = self.0.get("/").expect("no root folder") {
            return *size;
        }

        0
    }

    pub fn record_folder_sizes(&self, sizes: &mut Vec<u32>) {
        self.0
            .get("/")
            .expect("missing root folder")
            .record_folder_size(sizes)
    }
}

#[derive(Debug, Eq, PartialEq)]
enum FileDescriptor {
    Folder {
        name: FileName,
        parent: Option<FileName>,
        size: u32,
        content: HashMap<FileName, FileDescriptor>,
    },

    File {
        name: FileName,
        parent: FileName,
        size: u32,
    },
}

impl FileDescriptor {
    pub fn insert_folder(&mut self, path: &mut Iter<'_, &str>, parent: &str, name: &str) {
        let FileDescriptor::Folder {content, ..} = self else { return; };
        match path.next() {
            Some(p) => content
                .get_mut(&p.to_string())
                .expect("no parent found")
                .insert_folder(path, p, name),
            None => {
                content.insert(
                    name.to_string(),
                    FileDescriptor::Folder {
                        name: name.to_string(),
                        size: 0,
                        parent: Some(parent.to_string()),
                        content: HashMap::new(),
                    },
                );
            }
        }
    }

    pub fn insert_file(
        &mut self,
        path: &mut Peekable<Iter<'_, &str>>,
        parent: &str,
        name: &str,
        file_size: u32,
    ) {
        let FileDescriptor::Folder {content, size, ..} = self else { return; };
        *size += file_size;
        //dbg!(&name, size);
        let next_parent = path.next();

        match next_parent {
            // there's another folder in the path so we go deeper
            Some(next_parent) => {
                if let Some(data) = content.get_mut(&next_parent.to_string()) {
                    data.insert_file(path, next_parent, name, file_size);
                } else {
                    dbg!(self);
                    panic!("fuck");
                }
            }
            None => {
                // next there is no path, so this folder is last one, so we put file here
                content.insert(
                    name.to_string(),
                    FileDescriptor::File {
                        name: name.to_string(),
                        parent: parent.to_string(),
                        size: file_size,
                    },
                );
            }
        }
    }

    pub fn sum_folder_sizes(&self, counter: &mut u32) {
        let FileDescriptor::Folder {size, content, ..} = self else {
            return ;
        };

        if *size <= 100000 {
            *counter += size;
        }

        content
            .values()
            .for_each(|value| value.sum_folder_sizes(counter))
    }

    pub fn record_folder_size(&self, sizes: &mut Vec<u32>) {
        let FileDescriptor::Folder {size, content, ..} = self else {
            return;
        };

        sizes.push(*size);

        content.values().for_each(|v| v.record_folder_size(sizes));
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum TerminalLine<'i> {
    ChangeDir(&'i str),
    List,
    Directory(&'i str),
    File { size: u32, name: &'i str },
}

impl<'i> TerminalLine<'i> {
    fn from_str(s: &'i str) -> Result<Self, Infallible> {
        if s.starts_with("$ cd") {
            return Ok(TerminalLine::ChangeDir(s.split(' ').last().unwrap()));
        };

        if s.starts_with("$ ls") {
            return Ok(TerminalLine::List);
        }

        if s.starts_with("dir") {
            return Ok(TerminalLine::Directory(s.split(' ').last().unwrap()));
        }

        let split = s.split(' ').collect::<Vec<&str>>();

        Ok(TerminalLine::File {
            name: split[1],
            size: split[0].parse::<u32>().unwrap(),
        })
    }
}

#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> FileSystem {
    let mut path = FilePath(vec![]);
    let mut file_system = FileSystem(HashMap::new());
    input
        .lines()
        .for_each(|line| match TerminalLine::from_str(line) {
            Ok(TerminalLine::ChangeDir(dir)) if dir == ".." => {
                path.pop();
            }
            Ok(TerminalLine::ChangeDir(dir)) => {
                file_system.add_folder(&path, dir);
                path.push(dir);
            }
            Ok(TerminalLine::File { name, size }) => {
                file_system.add_file(&path, name, size);
            }
            _ => (),
        });
    file_system
}

#[aoc(day7, part1)]
pub fn part1(input: &FileSystem) -> u32 {
    let mut counter = 0u32;

    input.sum_folder_sizes_for(&mut counter);

    counter
}

const REQUIRED_SPACE: u32 = 30000000;
const TOTAL_SPACE: u32 = 70000000;

#[aoc(day7, part2)]
pub fn part2(input: &FileSystem) -> u32 {
    let used = input.used_space();
    let available = TOTAL_SPACE - used;
    let need_to_free = REQUIRED_SPACE - available;

    let mut folder_sizes = vec![];
    input.record_folder_sizes(&mut folder_sizes);
    let mut filtered_sizes = folder_sizes
        .into_iter()
        .filter(|size| *size > need_to_free)
        .collect::<Vec<u32>>();
    filtered_sizes.sort();
    *filtered_sizes
        .first()
        .expect("expected at least one folder size")
}
