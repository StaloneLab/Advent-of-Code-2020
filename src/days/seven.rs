extern crate petgraph;

use std::collections::HashMap;

use petgraph::prelude::NodeIndex;
use petgraph::graph::Graph;
#[allow(unused_imports)]
use petgraph::dot::Dot;
use petgraph::algo::has_path_connecting;
use petgraph::visit::EdgeRef;

fn input_parse(line: &String) -> (&str, HashMap<String, usize>) {
	let splitted_line: Vec<&str> = line.split(" bags contain").collect();
	let mut result: HashMap<String, usize> = HashMap::new();

	if splitted_line[1].contains("no other bags") {
		return (splitted_line[0], result);
	}

	for content in splitted_line[1].split(",") {
		let splitted_content: Vec<&str> = content[1..].split(" ").collect();
		let count: usize = splitted_content[0].parse().unwrap();
		let mut dep_name = splitted_content[1..].join(" ");

		if dep_name.ends_with('.') {
			dep_name = dep_name[..dep_name.len() - 1].to_string();
		}

		if dep_name.ends_with('s') {
			dep_name = dep_name[..dep_name.len() - 1].to_string();
		}

		dep_name = dep_name[..dep_name.len() - 4].to_string();

		result.insert(dep_name, count);
	}

	(splitted_line[0], result)
}

pub fn run(input: Vec<String>) {
	// - How can she pull off literally wearing a potato sack?
	// - It's a cute sack tho
	// (yes, the variable name is a reference to something stupid)
	let mut sack_graph = Graph::<String, usize>::new();
	let mut nodes: HashMap<String, NodeIndex> = HashMap::new();

	for (sack_type, sack_children) in input.iter().map(input_parse) {
		let sack_node = match nodes.get(sack_type) {
			Some(n) => *n,
			None => {
				let st = sack_type.to_string();
				let r = sack_graph.add_node(st.clone());
				nodes.insert(st, r);

				r
			}
		};

		for (child_type, child_weight) in sack_children {
			// Yes, I copy-pasted it without fear :D
			let child_node = match nodes.get(&child_type) {
				Some(n) => *n,
				None => {
					let r = sack_graph.add_node(child_type.clone());
					nodes.insert(child_type, r);

					r
				}
			};

			sack_graph.add_edge(sack_node, child_node, child_weight);
		}
	}

	// Part one
	let given_sack = *nodes.get("shiny gold").unwrap();
	let mut count = -1;

	for i in sack_graph.node_indices() {
		if has_path_connecting(&sack_graph, i, given_sack, Option::None) {
			count += 1;
		}
	}

	//println!("{}", Dot::new(&sack_graph));
	println!("Part one: {}", count);

	// Second part
	struct Recurse<'s> { f: &'s dyn Fn(&Recurse, NodeIndex) -> usize }

	let recurse_into = Recurse {
		f: &|recurse_into: &Recurse, node: NodeIndex| -> usize {
			let mut current_weight = 0;

			for e in sack_graph.edges(node) {
				let child_weight = (recurse_into.f)(recurse_into, e.target());
				current_weight += child_weight * e.weight();
			}

			current_weight + 1
		},
	};

	let final_weight = (recurse_into.f)(&recurse_into, given_sack);

	println!("Part two: {}", final_weight - 1);
}