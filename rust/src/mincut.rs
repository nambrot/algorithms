extern crate rand;
extern crate rayon;
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use rayon::prelude::*;

pub type Graph = Vec<(usize, usize)>;
pub fn parse_file(filename: &str) -> Graph {
    let mut s = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut s).unwrap();

    let mut g: Graph = vec![];

    for line in s.lines() {
        let mut raws: Vec<usize> =
            line.split_whitespace().map(|num: &str| num.parse::<usize>().unwrap()).collect();
        let node = raws[0];
        raws.remove(0);
        let mut edges: Vec<(usize, usize)> =
            raws.iter().map(|num: &usize| (node, num.clone())).collect();
        g.append(&mut edges);
    }

    g
}

pub fn karger(graph: &Graph) -> usize {
    let mut tries: Vec<usize> = (1..10000).into_par_iter().map(|_| mincut_run(graph)).collect();
    tries.sort();
    tries[0]
}

fn mincut_run(graph: &Graph) -> usize {
    let mut g: Graph = graph.clone();
    let mut nodes: Vec<usize> = graph.iter().map(|&(a, _)| a).collect();
    nodes.sort();
    nodes.dedup();

    loop {
        if nodes.len() <= 2 {
            break;
        }

        let (b, aval, bval) = random_nodes(&nodes);

        merge_edges(&mut g, aval, bval);
        replace_self_loops(&mut g);
        nodes.remove(b);
    }
    g.len()
}

fn random_nodes(nodes: &Vec<usize>) -> (usize, usize, usize) {
    let a = rand::thread_rng().gen_range(0, nodes.len());
    let mut b: usize;
    loop {
        b = rand::thread_rng().gen_range(0, nodes.len());
        if a != b {
            break;
        }
    }

    (b, nodes[a], nodes[b])
}

fn merge_edges(g: &mut Graph, aval: usize, bval: usize) {
    // replace in adjacency list
    let mut i = 0;
    loop {
        if i == g.len() {
            break;
        }

        match g[i] {
            (nodeval, edgeval) if nodeval == bval => g[i] = (aval, edgeval),
            (nodeval, edgeval) if edgeval == bval => g[i] = (nodeval, aval),
            _ => (),
        }

        assert!(g[i].0 != bval, "node value wasnt replaced");
        assert!(g[i].1 != bval, "edge value asnt replaced");

        i += 1;
    }
}

fn replace_self_loops(g: &mut Graph) {
    let mut i = 0;
    loop {
        if i == g.len() {
            break;
        }

        let (nodeval, edgeval) = g[i];
        if nodeval == edgeval {
            g.remove(i);
        } else {
            i += 1;
        }
    }
}
