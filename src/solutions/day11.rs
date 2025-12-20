use crate::Solve;
use petgraph::visit::EdgeRef;
use petgraph::{Directed, Graph};
use std::collections::HashMap;
type NodeIndex = petgraph::graph::NodeIndex;

pub struct Problem<'a> {
    graph: Graph<&'a str, (), Directed>,
}
impl Solve for Problem<'_> {
    /// Directed graph, paths from "you" to "out"
    #[allow(clippy::cast_possible_wrap)]
    fn p1(&mut self) -> i64 {
        self.count_paths(self.get_node("you"), self.get_node("out"), &mut HashMap::new()) as i64
    }

    /// svr to out, must pass through "dac" and "fft", 155 us
    #[allow(clippy::cast_possible_wrap)]
    fn p2(&mut self) -> i64 {
        let svr = self.get_node("svr");
        let out = self.get_node("out");
        let dac = self.get_node("dac");
        let fft = self.get_node("fft");

        // Paths svr -> dac -> fft -> out + svr -> fft -> dac -> out
        // Small optimization: only one middle path exists, saves unneeded calculations
        let dac_to_fft = self.count_paths(dac, fft, &mut HashMap::new());
        if dac_to_fft > 0 {
            let svr_to_dac = self.count_paths(svr, dac, &mut HashMap::new());
            let fft_to_out = self.count_paths(fft, out, &mut HashMap::new());
            return (svr_to_dac * dac_to_fft * fft_to_out) as i64;
        }

        let svr_to_fft = self.count_paths(svr, fft, &mut HashMap::new());
        let fft_to_dac = self.count_paths(fft, dac, &mut HashMap::new());
        let dac_to_out = self.count_paths(dac, out, &mut HashMap::new());

        (svr_to_fft * fft_to_dac * dac_to_out) as i64
    }
}

impl<'a> Problem<'a> {
    pub fn new(data: &'a [String]) -> Self {
        let mut graph: Graph<&str, (), Directed> = Graph::new();

        for line in data {
            let parts = line.split(':').collect::<Vec<&str>>();
            let targets = parts[1].split_whitespace().collect::<Vec<&str>>();

            let source_index = Self::get_or_add_node(&mut graph, parts[0]);

            for target in targets {
                let target_index = Self::get_or_add_node(&mut graph, target);
                graph.add_edge(source_index, target_index, ());
            }
        }

        Problem { graph }
    }

    /// Helper to find or add a node by name
    fn get_or_add_node(graph: &mut Graph<&'a str, (), Directed>, name: &'a str) -> NodeIndex {
        graph
            .node_indices()
            .find(|&i| graph[i] == name)
            .unwrap_or_else(|| graph.add_node(name))
    }
    fn get_node(&self, name: &str) -> NodeIndex {
        self.graph.node_indices().find(|&i| self.graph[i] == name).unwrap()
    }

    /// Count paths using memoized DFS - linear complexity
    fn count_paths(&self, from: NodeIndex, to: NodeIndex, memo: &mut HashMap<NodeIndex, usize>) -> usize {
        // We found ourselves,  one valid path
        if from == to {
            return 1;
        }

        // If our start node has path counts calculated, return it
        if let Some(&cached) = memo.get(&from) {
            return cached;
        }

        // Explore neighbors and count paths recursively
        let mut count = 0;
        for edge in self.graph.edges(from) {
            count += self.count_paths(edge.target(), to, memo);
        }

        // Cache the computed path count
        memo.insert(from, count);

        // Return the total path count from 'from' to 'to'
        count
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod test {
    use super::*;
    use crate::load_file;
    const ANSWERS: [i64; 4] = [5, 2, 523, 517_315_308_154_944];

    #[test] fn p1() { assert_eq!(Problem::new(&load_file("input/11_test.txt")).p1(), ANSWERS[0]); }
    #[test] fn p2() { assert_eq!(Problem::new(&load_file("input/11_test2.txt")).p2(), ANSWERS[1]); }
    #[test] fn f1() { assert_eq!(Problem::new(&load_file("input/11.txt")).p1(), ANSWERS[2]); }
    #[test] fn f2() { assert_eq!(Problem::new(&load_file("input/11.txt")).p2(), ANSWERS[3]); }
}
