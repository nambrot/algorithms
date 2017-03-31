use std;
use std::fs::File;
use std::io::prelude::*;
use std::collections::*;
use std::cmp::Ordering;
pub type Node = usize;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Edge {
    node1: Node,
    node2: Node,
    weight: isize,
}

pub type Edges = Vec<Edge>;
pub type Graph = HashMap<Node, Edges>;
pub type Tree = Edges;

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight).reverse()
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Edge) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


pub fn parse_file(filename: &str) -> Graph {
    let mut s = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut s).unwrap();
    let mut graph: Graph = HashMap::new();
    for line in s.lines().skip(1) {
        let nums: Vec<isize> =
            line.split_whitespace().map(|num: &str| num.parse::<isize>().unwrap()).collect();
        let node1: Node = nums[0] as usize;
        let node2: Node = nums[1] as usize;
        let edge: Edge = Edge { node1: node1, node2: node2, weight: nums[2] };
        insert_edge(&mut graph, node1, edge.clone());
        insert_edge(&mut graph, node2, edge.clone());
    }
    graph
}

pub fn minimum_spanning_tree(graph: &Graph) -> Tree {
    let mut tree: Tree = vec![];
    let mut added_nodes: HashSet<Node> = HashSet::new();

    let first_node = graph.keys().next().unwrap().clone();
    added_nodes.insert(first_node);

    let mut edges_to_explore: BinaryHeap<Edge> = BinaryHeap::new();
    let edges_to_add = graph.get(&first_node).unwrap();
    for edge in edges_to_add {
        edges_to_explore.push(edge.clone())
    }

    while added_nodes.len() != graph.len() {
        match edges_to_explore.pop() {
            Some(ref edge) if added_nodes.contains(&edge.node1) && !added_nodes.contains(&edge.node2) => {
                tree.push(edge.clone());
                added_nodes.insert(edge.node2);
                let edges_to_add = graph.get(&edge.node2).unwrap();
                for edge in edges_to_add {
                    edges_to_explore.push(edge.clone())
                }
            },
            Some(ref edge) if !added_nodes.contains(&edge.node1) && added_nodes.contains(&edge.node2) => {
                tree.push(edge.clone());
                added_nodes.insert(edge.node1);
                let edges_to_add = graph.get(&edge.node1).unwrap();
                for edge in edges_to_add {
                    edges_to_explore.push(edge.clone())
                }
            },
            _ => (),
        }

        assert!(edges_to_explore.len() > 0, "something went wong")
    }

    // Verify
    let mut verify_set: HashSet<Node> = HashSet::new();
    for edge in tree.iter() {
        verify_set.insert(edge.node1);
        verify_set.insert(edge.node2);
    }
    assert!(verify_set.len() == graph.len(), "Tree is not spanning");

    tree
}

fn insert_edge(graph: &mut Graph, node: Node, edge: Edge) {
    let edges = graph.entry(node).or_insert(vec![]);
    edges.push(edge);
}
