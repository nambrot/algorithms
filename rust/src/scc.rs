use std::fs::File;
use std::io::prelude::*;
use std::collections::*;
use pbr::ProgressBar;
use std::cmp::Ordering;
pub type Node = u32;
pub type Edges = Vec<Node>;
pub type Graph = HashMap<Node, Edges>;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Component {
    leader: Node,
    nodes: HashSet<Node>,
    size: usize,
}

impl Ord for Component {
    fn cmp(&self, other: &Self) -> Ordering {
        self.size.cmp(&other.size)
    }
}

impl PartialOrd for Component {
    fn partial_cmp(&self, other: &Component) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn parse_file(filename: &str) -> Graph {
    let mut s = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut s).unwrap();

    let mut g: Graph = HashMap::new();
    for line in s.lines().take(10000) {
        let raws: Vec<u32> =
            line.split_whitespace().map(|num: &str| num.parse::<Node>().unwrap()).collect();
        insert_edge(&mut g, raws[0], raws[1]);
    }

    g
}

pub fn strongest_connected_components(graph: &Graph, n: u32) -> Vec<u32> {
    let reversed_graph = reverse(graph);
    //   DFS through the reversed graph
    let mut explored_nodes: HashSet<Node> = HashSet::new();
    let mut second_pass_stack: Vec<Node> = vec![];

    for (starting_node, _) in reversed_graph.iter() {
        if !explored_nodes.contains(&starting_node) {
            let mut stack: Vec<Node> = vec![];
            stack.push(starting_node.clone());
            while !stack.is_empty() {
                let current_node = stack.pop().unwrap();
                if explored_nodes.contains(&current_node) { continue; }
                explored_nodes.insert(current_node);

                let empty: Vec<u32> = Vec::new();
                let mut edges: Vec<u32> = reversed_graph.get(&current_node)
                    .unwrap_or(&empty)
                    .clone()
                    .into_iter()
                    .filter(|node: &Node| !explored_nodes.contains(node))
                    .collect::<Vec<u32>>()
                    .to_vec();
                if edges.is_empty() {
                    second_pass_stack.push(current_node);
                } else {
                    stack.push(current_node);
                    stack.append(&mut edges);
                }
            }
        }
    }

    let mut components: BinaryHeap<Component> = BinaryHeap::new();
    explored_nodes = HashSet::new();
    second_pass_stack.reverse();
    for leader in second_pass_stack.iter() {
        if !explored_nodes.contains(&leader) {
            let mut component: HashSet<Node> = HashSet::new();
            let mut stack: Vec<Node> = vec![];
            stack.push(leader.clone());
            while !stack.is_empty() {
                let current_node = stack.pop().unwrap();
                if explored_nodes.contains(&current_node) { continue; }
                explored_nodes.insert(current_node);

                let empty: Vec<u32> = Vec::new();
                let mut edges: Vec<u32> = graph.get(&current_node)
                    .unwrap_or(&empty)
                    .clone()
                    .into_iter()
                    .filter(|node: &Node| !explored_nodes.contains(node))
                    .collect::<Vec<u32>>()
                    .to_vec();
                if edges.is_empty() {
                    component.insert(current_node);
                } else {
                    stack.push(current_node);
                    stack.append(&mut edges);
                }
            }
            components.push(Component {
                leader: leader.clone(),
                size: component.len(),
                nodes: component.clone(),
            });


        }
    }

    // println!("{:?}", graph);
    // // println!("{:?}", reversed_graph);
    // println!("{:?}", second_pass_stack);
    // println!("{:?}", components);
    // assert strongly connectedness
    for component in &components {
        assert!(is_strongly_connected(graph, &component),
            "is not a component {:?}", component);
    }

    let mut components_to_return = vec![];
    for component in &components {
        if components_to_return.len() as u32 >= n { break; }
        components_to_return.push(component);
    }

    components_to_return.iter().map(|component| component.size as u32).collect()
}

fn is_strongly_connected(graph: &Graph, component: &Component) -> bool {
    for node in component.nodes.iter() {
        !search(graph, &component.leader, node) || !search(graph, node, &component.leader);
    }
    true
}

fn search(graph: &Graph, s: &Node, v: &Node) -> bool {
    let mut stack: Vec<Node> = vec![];
    let mut explored_nodes = HashSet::new();
    stack.push(s.clone());
    while let Some(node) = stack.pop() {
        if &node == v {
            return true;
        }
        explored_nodes.insert(node);

        let empty: Vec<u32> = Vec::new();
        let mut edges: Vec<u32> = graph.get(&node)
            .unwrap_or(&empty)
            .clone()
            .into_iter()
            .filter(|node: &Node| !explored_nodes.contains(node))
            .collect::<Vec<u32>>()
            .to_vec();
        if !edges.is_empty() {
            stack.push(node);
            stack.append(&mut edges);
        }
    }
    assert!(false, "{} couldnt find a path to {}", s, v);
    false
}

fn reverse(graph: &Graph) -> Graph {
    let mut reversed_graph: Graph = HashMap::new();
    for (node, edges) in graph {
        for target in edges {
            insert_edge(&mut reversed_graph, target.clone(), node.clone());
        }
    }
    reversed_graph
}

fn insert_edge(graph: &mut Graph, src: u32, target: u32) {
    let edges = graph.entry(src).or_insert(vec![]);
    edges.push(target);
}
