use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Node {
    name: String,
    size: usize,
    children: Vec<Node>
}

impl Node {
    fn root() -> Node {
        Node {
            name: "/".to_string(),
            size: 0,
            children: Vec::new(),
        }
    }

    fn from(name: String, size: Option<usize>) -> Node {
        let mut size = size; // may cause confusion
        if size == None {
            size = Some(0);
        }
        
        Node {
            name,
            size: size.unwrap(),
            children: Vec::new(),
        }
    }

    fn get_ptr(&mut self, name: String) -> *mut Node {
        for node in &mut self.children {
            if node.name == name {
                return node;
            }
        }
        
        self.children.push(Node::from(name.clone(), None));
        let node = self.get_ptr(name);
        return node;
    }

    fn calculate_node_sizes(&mut self) {
        // calculate correct sizes with recursion
        for node in &mut self.children {
            if node.children.len() > 0 {
                node.calculate_node_sizes();
                node.size = node.children.iter().map(|x| x.size).sum::<usize>();
            }
        }
    }

    fn calculate_root_size(&mut self) {
        // calculate correct size of root
        self.size = self.children.iter().map(|x| x.size).sum::<usize>();
    }

    fn get_nodes_under_size(&self, size: usize) -> Vec<&Node> {
        let mut nodes: Vec<&Node> = Vec::new();
        for node in &self.children {
            if node.children.len() > 0 && node.size < size{
                nodes.push(node);
                if node.size == 0 {
                    // panic!("Size is 0 for node: {:#?}", node);
                }
            }
            nodes.append(&mut node.get_nodes_under_size(size));
        }
        return nodes;
    }

    fn total_num_dirs(&self) -> usize {
        let mut num = 0;
        for node in &self.children {
            if node.children.len() > 0 {
                num += 1;
            }
            num += node.total_num_dirs();
        }
        return num;
    }
}

fn main() {
    let input = read_to_string("input-7.txt").unwrap();
    let mut lines: Vec<&str> = input.lines().collect();

    let mut fs = Node::root();
    lines.remove(0);

    let mut pwd = &mut fs;
    let mut dir_stack: Vec<&mut Node> = Vec::new();
    for command in lines {
        let command: Vec<&str> = command.split(" ").collect();

        if command[0] != "$" {
            match command[0] {
                "dir" => {
                    pwd.children.push(Node::from(command[1].to_string(), None));
                },

                _ => {
                    let size = command[0].parse().unwrap();
                    pwd.children.push(Node::from(command[1].to_string(), Some(size)));
                    pwd.size += size;
                }
            }
        } else {
            match command[1] {
                "cd" => {
                    if command[2] == ".." {
                        pwd = dir_stack.pop().unwrap();
                    } else {
                        unsafe { 
                            let node = pwd.get_ptr(command[2].to_string()).as_mut().unwrap(); 
                            dir_stack.push(pwd);
                            pwd = node;
                        }
                        
                    }
                },
    
                "ls" => (),
    
                _ => { panic!("Unknown command: {:?}", command); }
            }
        }
    }

    fs.calculate_node_sizes();
    fs.calculate_root_size();

    let vec = fs.get_nodes_under_size(100000);
    let mut sum = 0;

    println!("{:#?}", fs);

    for node in vec {
        println!("{}: {}", node.name, node.size);
        sum += node.size;
    }

    println!("\nSum: {}", sum);

    println!("Total number of nodes: {}", fs.total_num_dirs());
}