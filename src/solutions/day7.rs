
#[derive(Debug)]
#[derive(Clone)]
struct Node<T>
where
    T: PartialEq + std::fmt::Debug,
{
    idx: usize,
    val: T,
    isdir: bool,
    parent: Option<usize>,
    children: Vec<usize>,
    size: Option<u32>
}

impl<T> Node<T>
where
    T: PartialEq + std::fmt::Debug,
{
    fn new(idx: usize, val: T, isdir: bool, size: Option<u32>, parent: Option<usize>) -> Self {
        Self {
            idx,
            val,
            isdir,
            parent,//: None,
            children: vec![],
            size
        }
    }
}

#[derive(Debug, Default)]
struct ArenaTree<T>
where
    T: PartialEq + std::fmt::Debug,
{
    arena: Vec<Node<T>>,
}

impl<T> ArenaTree<T>
where
    T: PartialEq + std::fmt::Debug,
{
    fn node(&mut self, val: T, isdir: bool, size: Option<u32>, parent: Option<usize>) -> usize {
        //first see if it exists
        for node in &self.arena {
            if node.val == val && node.parent == parent {
                return node.idx;
            }
        }
        // Otherwise, add new node
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, val, isdir, size, parent));
        idx
    }

    fn set_folders_size(&mut self) {
        loop {
            let mut has_empty_folders: bool = false;
            for idx in 0..self.arena.len() {
                if self.arena[idx].isdir && self.arena[idx].size == Some(0) {
                    let mut has_empty_folder_child: bool = false;
                    let mut folder_size: u32 = 0;
                    for child in &self.arena[idx].children {
                        if self.arena[*child].isdir && self.arena[*child].size == Some(0) {
                            has_empty_folder_child = true;
                            has_empty_folders = true;
                            break;
                        }
                        else {
                            folder_size += self.arena[*child].size.unwrap();
                        }
                    }
                    if !has_empty_folder_child {
                        self.arena[idx].size = Some(folder_size);
                    }
                }
            }
            if !has_empty_folders { break; }
        }
    }
}

pub fn solve(part: u8, input: &String) -> String {
    let vecstr: Vec<&str> = input.lines().collect();
    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut current_index: usize = 0;

    for li in vecstr {
        if li == "$ cd /" {
            current_index = tree.node("/".into(), true, Some(0), Some(0));
            tree.arena[current_index].size = Some(0);
        }
        else if li.starts_with("$ cd ") && li != "$ cd .." {
            let mut spl_folder = li.split(" ");
            let (_, _, folder_name) = (spl_folder.next().unwrap(), spl_folder.next().unwrap(), spl_folder.next().unwrap());
            for idx in &tree.arena[current_index].children {
                if tree.arena[*idx].val == folder_name {
                    current_index = *idx;
                    break;
                }
            }
        }
        else if li == "$ cd .." {
            current_index = tree.arena[current_index].parent.unwrap();
        }
        else if li.starts_with("dir") {
            let mut spl_folder = li.split(" ");
            let (_, folder_name) = (spl_folder.next().unwrap(), spl_folder.next().unwrap());
            let idx_dir = tree.node(folder_name.into(), true, Some(0), Some(current_index));
            tree.arena[current_index].children.push(idx_dir);
        }
        else if li[0..1].chars().all(char::is_numeric) {
            let mut spl_file = li.split(" ");
            let (file_size, file_name) = (spl_file.next().unwrap().parse::<u32>().unwrap(), spl_file.next().unwrap());
            let idx_file = tree.node(file_name.into(), false, Some(file_size), Some(current_index));
            tree.arena[current_index].children.push(idx_file);
        }
    }

    tree.set_folders_size();

    let mut idx: usize = 0;
    let mut sum_size: u32 = 0;

    for _ in &tree.arena {
        if tree.arena[idx].isdir && tree.arena[idx].size <= Some(100000) {
            sum_size += tree.arena[idx].size.unwrap();
        }
        idx += 1;
    }
    if part == 1 {
        return sum_size.to_string();
    }

    return String::from("Wrong value for part");
}