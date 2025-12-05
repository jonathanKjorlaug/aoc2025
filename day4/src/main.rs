#![allow(dead_code)]
use std::fmt;
use std::{fs, vec};

enum InputType {
    Dummy,
    Real,
}

#[derive(Clone)]
struct Node {
    row: usize,
    column: usize,
    neighbours: Vec<usize>,
    // neighbours: us,
}

impl Node {
    fn add_neighbour(&mut self, node_id: usize) {
        self.neighbours.push(node_id);
    }

    fn remove_node(&mut self, id: usize, node_list: &mut Vec<Node>) {
        for &nid in &self.neighbours {
            println!("nid: {} id: {}", nid, id);
            println!("Neighbours {:?}", node_list[nid].neighbours);
            node_list[nid].neighbours.retain(|&i| i != id);

            println!("Neighbours {:?}", node_list[nid].neighbours);
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("i", &self.row)
            .field("j", &self.column)
            .field("neighbours", &self.neighbours)
            .finish()
    }
}

fn construct_graph(content: String) -> Vec<Node> {
    let mut nodes: Vec<Node> = vec![];

    for (i, line) in content.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '@' {
                nodes.push(Node {
                    row: i,
                    column: j,
                    neighbours: vec![],
                });
            }
        }
    }

    for i in 0..nodes.len() {
        let (a_row, a_col) = (nodes[i].row, nodes[i].column);
        for j in 0..nodes.len() {
            if i == j {
                continue;
            }
            let bnode = &nodes[j];
            if a_row.abs_diff(bnode.row) <= 1 && a_col.abs_diff(bnode.column) <= 1 {
                nodes[i].add_neighbour(j);
            }
        }
    }

    return nodes;
}

fn first_part(input: InputType) {
    let path = match input {
        InputType::Dummy => "dummy_input.txt",
        InputType::Real => "input.txt",
    };

    let content = fs::read_to_string(path).expect("Test");

    let nodes = construct_graph(content);

    let lonely_number = nodes
        .iter()
        .filter(|node| node.neighbours.len() < 4)
        .count();

    println!("{lonely_number}");
}

fn remove_nodes(nodes: Vec<Node>, all_nodes: &Vec<Node>) -> Vec<Node> {
    let lonely_indices: Vec<usize> = nodes
        .iter()
        .enumerate()
        .filter(|(_, node)| node.neighbours.len() < 4)
        .map(|(i, _)| i)
        .collect();

    if lonely_indices.is_empty() {
        return nodes;
    }

    println!("{}", lonely_indices.len());

    // Make a working copy
    let mut next_nodes: Vec<Node> = nodes.iter().cloned().collect();

    // Build a map from global index (all_nodes) -> index in next_nodes
    let mut global_to_next: std::collections::HashMap<_, _> = next_nodes
        .iter()
        .enumerate()
        .map(|(i, n)| {
            let global_idx = all_nodes
                .iter()
                .position(|x| x.row == n.row && x.column == n.column)
                .unwrap();
            (global_idx, i)
        })
        .collect();

    for &i in lonely_indices.iter().rev() {
        let global_idx = all_nodes
            .iter()
            .position(|n| n.row == next_nodes[i].row && n.column == next_nodes[i].column)
            .unwrap();

        // Remove this node from all neighbours in next_nodes
        let neighbours = next_nodes[i].neighbours.clone();
        for nid in neighbours {
            if let Some(&idx) = global_to_next.get(&nid) {
                next_nodes[idx].neighbours.retain(|&x| x != global_idx);
            }
        }

        next_nodes.remove(i);

        // Rebuild the mapping after removal
        global_to_next = next_nodes
            .iter()
            .enumerate()
            .map(|(i, n)| {
                let global_idx = all_nodes
                    .iter()
                    .position(|x| x.row == n.row && x.column == n.column)
                    .unwrap();
                (global_idx, i)
            })
            .collect();
    }

    remove_nodes(next_nodes, all_nodes)
}

// fn remove_nodes(nodes: Vec<Node>, all_nodes: &mut Vec<Node>) -> Vec<Node> {
//     // let lonely_nodes = nodes.iter().filter(|node| node.neighbours.len() < 4);
//
//     let lonely_indices: Vec<usize> = nodes
//         .iter()
//         .enumerate()
//         .filter(|(_, node)| node.neighbours.len() < 4)
//         .map(|(i, _)| i)
//         .collect();
//
//     if lonely_indices.is_empty() {
//         return nodes;
//     }
//
//     let mut next_nodes: Vec<Node> = nodes.iter().cloned().collect();
//
//     for i in lonely_indices.iter().rev() {
//         let global_idx = all_nodes
//             .iter()
//             .position(|n| n.row == nodes[*i].row && n.column == nodes[*i].column)
//             .unwrap();
//
//         next_nodes[*i].remove_node(global_idx, all_nodes);
//         next_nodes.remove(*i);
//     }
//
//     return remove_nodes(next_nodes, all_nodes);
// }

fn second_part(input: InputType) {
    let path = match input {
        InputType::Dummy => "dummy_input.txt",
        InputType::Real => "input.txt",
    };

    let content = fs::read_to_string(path).expect("Test");

    let nodes = construct_graph(content);
    let start_length = nodes.len();

    println!("Started removing nodes");
    println!(
        "{}",
        start_length - remove_nodes(nodes.clone(), &mut nodes.clone()).len()
    )
}

fn main() {
    // first_part(InputType::Dummy);
    // first_part(InputType::Real);
    second_part(InputType::Real)
}
