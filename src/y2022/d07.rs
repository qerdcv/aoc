use std::collections::HashMap;

#[derive(PartialEq)]
enum FSType {
    Dir,
    File,
}

struct FSNode {
    t: FSType,
    children: HashMap<String, usize>,
    parent: usize,
    size: usize,
}

struct FileSystem {
    nodes: Vec<FSNode>
}

impl FileSystem {
    fn new(size: usize) -> Self {
        let root = FSNode {
            t: FSType::Dir,
            children: HashMap::new(),
            parent: 0,
            size,
        };

        Self { nodes: vec![root] }
    }

    fn add_node(&mut self, t: FSType, parent_idx: usize, size: usize) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(FSNode {
            children: HashMap::new(),
            parent: parent_idx,
            t,
            size,
        });

        idx
    }
}

fn parse_fs(input: &str) -> FileSystem {
    let mut fs = FileSystem::new(0);
    let mut current = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let p = line.split_ascii_whitespace().collect::<Vec<_>>();
        match p.as_slice() {
            ["$", "cd", "/"] => {},
            ["$", "cd", ".."] => {
                    let parrent = fs.nodes[current].parent;
                    fs.nodes[parrent].size += fs.nodes[current].size;
                    current = parrent;
            },
            ["$", "cd", folder] => {
                let folder = *folder;
                match fs.nodes[current].children.get(folder.into()) {
                    Some(&new_idx) => current = new_idx,
                    None => { // insert new children node
                        current = fs.add_node(FSType::Dir, current, 0);
                    }
                }
            },
            ["$", "ls"] | ["dir", _] => {},
            [size, _] => {
                let size = size.parse::<usize>().unwrap();
                fs.add_node(FSType::File, current, size);
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
    let total_free_space = TOTAL_SPACE - fs.nodes[0].size;
    let need_to_free = UPDATE_SIZE - total_free_space;

    fs.nodes
        .iter()
        .filter(|n| n.t == FSType::Dir && n.size >= need_to_free)
        .map(|n| n.size)
        .min()
        .unwrap_or(0) as i64
}
