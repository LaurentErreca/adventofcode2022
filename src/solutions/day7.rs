
#[derive(Debug)]
#[derive(Clone)]
struct Node<T>
where
    T: PartialEq,
{
    idx: usize,
    val: T,
    parent: Option<usize>,
    children: Vec<usize>,
    size: Option<u32>
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(idx: usize, val: T, size: Option<u32>) -> Self {
        Self {
            idx,
            val,
            parent: None,
            children: vec![],
            size
        }
    }
}

#[derive(Debug, Default)]
struct ArenaTree<T>
where
    T: PartialEq,
{
    arena: Vec<Node<T>>,
}

impl<T> ArenaTree<T>
where
    T: PartialEq,
{
    fn node(&mut self, val: T, size: Option<u32>) -> usize {
        //first see if it exists
        for node in &self.arena {
            if node.val == val {
                return node.idx;
            }
        }
        // Otherwise, add new node
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, val, size));
        idx
    }

    fn get_folder_size(&self, node: usize) -> u32 {
        let mut total_size: u32 = 0;
        for child in &self.arena[node].children {
            //println!("node : {} - child idx : {}", node, child);
            match self.arena[*child].size {
                Some(0) => total_size += self.get_folder_size(*child),
                Some(size) => total_size += size,
                None => println!("None")
            }
        }
        return total_size.try_into().unwrap();
    }

}

pub fn solve(part: u8, input: &String) -> String {
    let vecstr: Vec<&str> = input.lines().collect();
    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut current_index: usize = 0;

    for li in vecstr {
        //println!("Line : {}", li);
        if li == "$ cd /" {
            current_index = tree.node("/".into(), None);
            tree.arena[current_index].size = Some(0);
            //println!("cd / => Current index : {}", current_index);
        }
        else if li.starts_with("$ cd ") && li != "$ cd .." {
            let mut spl_folder = li.split(" ");
            let (_, _, folder_name) = (spl_folder.next().unwrap(), spl_folder.next().unwrap(), spl_folder.next().unwrap());
            current_index = tree.node(folder_name.into(), None);
            //println!("Current index : {}", current_index);
        }
        else if li == "$ cd .." {
            current_index = tree.arena[current_index].parent.unwrap();
        }
        else if li.starts_with("dir") {
            let mut spl_folder = li.split(" ");
            let (_, folder_name) = (spl_folder.next().unwrap(), spl_folder.next().unwrap());
            //println!("Folder name : {:?}", folder_name);
            let idx_dir = tree.node(folder_name.into(), None);
            tree.arena[current_index].children.push(idx_dir);
            tree.arena[idx_dir].parent = Some(current_index);
            tree.arena[idx_dir].size = Some(0);
            //println!("Current folder index : {}", current_index);
        }
        else if li[0..1].chars().all(char::is_numeric) {
            let mut spl_folder = li.split(" ");
            let (file_size, folder_name) = (spl_folder.next().unwrap().parse::<u32>().unwrap(), spl_folder.next().unwrap());
            //println!("File : {:?} - size : {:?}", li, file_size);
            let idx_file = tree.node(folder_name.into(), None);
            tree.arena[current_index].children.push(idx_file);
            tree.arena[idx_file].parent = Some(current_index);
            tree.arena[idx_file].size = Some(file_size);
            //println!("Current file index : {}", idx_file);
        }
    }


    let mut idx: usize = 0;
    //let mut idx: usize = tree.arena.len();
    let mut sum_size: u32 = 0;
    println!("Len arena : {}", tree.arena.len());

    /* for _ in tree.arena.clone() {
        idx -= 1;
        if tree.arena[idx].size == Some(0) {
            tree.arena[idx].size = Some(tree.get_folder_size(idx));
        }
        //println!("Index : {}", idx);
        //println!("Size : {}", tree.get_folder_size(idx));
        //if tree.arena[idx].size == Some(0) && tree.get_folder_size(idx) <= 100000 {
            //println!("Folder with size less than 100000 : {:?} - size : {}", tree.arena[idx].val, tree.get_folder_size(idx));
        //    sum_size += tree.get_folder_size(idx);
        //}
        //println!("Idx : {}", idx);
    } */

    for _ in &tree.arena {
        if tree.arena[idx].size == Some(0) && tree.get_folder_size(idx) <= 100000 {
            //println!("Folder with size less than 100000 : {:?} - size : {}", tree.arena[idx].val, tree.get_folder_size(idx));
            sum_size += tree.get_folder_size(idx);
        }
        idx += 1;
    }
    //println!("Total size : {}", sum_size);
    if part == 1 {
        return sum_size.to_string();
    }

    return String::from("Wrong value for part");
}