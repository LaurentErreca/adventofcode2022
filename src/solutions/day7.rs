use std::collections::HashSet;

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
    size: Option<i32>
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(idx: usize, val: T, size: Option<i32>) -> Self {
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
    fn node(&mut self, val: T, size: Option<i32>) -> usize {
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
    fn size(&self) -> usize {
        self.arena.len()
    }
    fn edges(&self) -> usize {
        self.arena
            .iter()
            .fold(0, |acc, node| acc + node.children.len())
    }

    fn depth(&self, idx: usize) -> usize {
        match self.arena[idx].parent {
            Some(id) => 1 + self.depth(id),
            None => 0,
        }
    }

    fn node_size(&self, idx: usize) -> Option<i32> {
        return self.arena[idx].size;
    }

    fn get_parent_idx(&self, node: usize) -> Option<usize> {
        self.arena[node].parent
    }

/*     fn get_parent_name(&self, node: usize) -> Option<T> {
        let aren: usize = (self.arena[node].parent.unwrap() as usize).clone();
        Some(self.arena[aren].val)
    } */
}

pub fn solve(part: u8, input: &String) -> String {
    let vecstr: Vec<&str> = input.lines().collect();

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let root = tree.node("/".into(), None);
    let dir_a = tree.node("a".into(), None);
    let dir_b = tree.node("b".into(), None);
    let dir_e = tree.node("e".into(), None);
    let file_c = tree.node("c".into(), Some(1234));
    let file_d = tree.node("d".into(), Some(999));
    let file_f = tree.node("f".into(), None);//, Some(1999));
    tree.arena[root].children.push(dir_a);
    tree.arena[root].children.push(dir_b);
    tree.arena[root].children.push(file_c);
    //tree.arena[dir_a].children.push(file_c);
    tree.arena[file_c].parent = Some(dir_a);
    tree.arena[dir_e].parent = Some(dir_a);
    tree.arena[file_f].parent = Some(dir_e);


    println!(
        "Total nodes: {}\nTotal edges: {}\nDepth of 'file_f': {}\nSize file_f: {:?}\nParent of file_f: {}\nParent name of file_f: {:?}\nChildrens of dir_a: {:?}",
        tree.size(),
        tree.edges(),
        tree.depth(file_f),
        match tree.node_size(file_f) {
            Some(size) => size,
            None => 0
        },
        match tree.get_parent_idx(file_f) {
            Some(idx) => idx,
            None => 0
        },
        match tree.get_parent_idx(file_f) {
            Some(idx) => tree.arena[idx].val.clone(),
            None => "No parent".to_string()
        },
        tree.arena[root].children
    );

    return String::from("Wrong value for part");
}