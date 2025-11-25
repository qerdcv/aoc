use std::{collections::{HashMap, VecDeque}, fmt::Display};

#[derive(PartialEq)]
enum FSType {
    Dir,
    File,
}

impl Display for FSType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FSType::Dir => write!(f, "dir"),
            FSType::File => write!(f, "file"),
        }
    }
}

struct FSNode {
    t: FSType,
    name: String,
    children: HashMap<String, usize>,
    parent: usize,
    size: usize,
}

struct FileSystem {
    nodes: Vec<FSNode>
}

impl FileSystem {
    fn new(name: impl Into<String>, size: usize) -> Self {
        let root = FSNode {
            t: FSType::Dir,
            name: name.into(),
            children: HashMap::new(),
            parent: 0,
            size,
        };

        Self { nodes: vec![root] }
    }

    fn add_node(&mut self, t: FSType, name: impl Into<String>, parent_idx: usize, size: usize) -> usize {
        let idx = self.nodes.len();
        let name: String = name.into();
        self.nodes.push(FSNode {
            name: name.clone(),
            children: HashMap::new(),
            parent: parent_idx,
            t,
            size,
        });
        self.nodes[parent_idx].children.insert(name, idx);

        idx
    }

    fn print(&self) {
        let mut print_q = VecDeque::new();
        print_q.push_back((0usize, 0usize));

        while let Some((i, depth)) = print_q.pop_front() {
            let node = &self.nodes[i];
            println!("{} - {} ({}, size: {})", " ".repeat(depth * 2), node.name, node.t, node.size);

            for (_, &ch_i) in &node.children {
                print_q.push_front((ch_i, depth + 1));
            }
        }
    }
}

fn parse_fs(input: &str) -> FileSystem {
    let mut fs = FileSystem::new("/", 0);
    let mut current = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let p = line.split_ascii_whitespace().collect::<Vec<_>>();
        match p.as_slice() {
            ["$", "cd", folder] => {
                let folder = *folder;
                if folder == "/" { // skip root
                    continue;
                }

                if folder == ".." {
                    let parrent = fs.nodes[current].parent;
                    fs.nodes[parrent].size += fs.nodes[current].size;
                    current = parrent;
                    continue;
                }

                match fs.nodes[current].children.get(folder.into()) {
                    Some(&new_idx) => current = new_idx,
                    None => { // insert new children node
                        current = fs.add_node(FSType::Dir, folder, current, 0);
                    }
                }
            },
            ["$", "ls"] => {},
            ["dir", _] => {},
            [size, file_name] => {
                let size = size.parse::<usize>().unwrap();
                fs.add_node(FSType::File, *file_name, current, size);
                fs.nodes[current].size += size;
            },
            _ => unreachable!(),
        }
    }

    let parrent = fs.nodes[current].parent;
    fs.nodes[parrent].size += fs.nodes[current].size;
    fs
}


pub fn p1(input: &str) -> i64 {
    let fs = parse_fs(input);
    
    fs.nodes
        .iter()
        .filter(|n| n.t == FSType::Dir && n.size <= 100_000)
        .map(|n| n.size)
        .sum::<usize>() as i64
}

pub fn p2(input: &str) -> i64 {
    const TOTAL_SPACE: usize = 70_000_000;
    const UPDATE_SIZE: usize = 30_000_000;

    let fs = parse_fs(input);
    fs.print();
    let total_free_space = TOTAL_SPACE - fs.nodes[0].size;
    let need_to_free = UPDATE_SIZE - total_free_space;

    fs.nodes
        .iter()
        .filter(|n| n.t == FSType::Dir && n.size >= need_to_free)
        .map(|n| n.size)
        .min()
        .unwrap_or(0) as i64
}
