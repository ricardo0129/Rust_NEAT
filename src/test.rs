#[cfg(test)]
mod tests {
    use std::assert_eq;

    use crate::{activation::ignore, genome::Genome, population::Population};
    #[test]
    fn fully_connected() {
        let mut g = Genome::new(4, 4, ignore);
        for i in 0..4 {
            for j in 0..4 {
                g.add_edge(i, 5 + j, i * 4 + j, 1.0, true);
            }
        }
        let input: Vec<f64> = vec![1.0; 4 as usize];
        let output = g.evaluate(&input);
        let expected = vec![4.0, 4.0, 4.0, 4.0];
        assert_eq!(output, expected);
    }

    #[test]
    fn split_node_working() {
        let mut g = Genome::new(1, 1, ignore);
        g.add_edge(0, 2, 1, 10.0, true);
        g.split_edge(0, 2, 3, 10);
        let input: Vec<f64> = vec![1.0; 1 as usize];
        let output = g.evaluate(&input);
        let expected = vec![10.0];
        assert_eq!(output, expected);
    }

    #[test]
    fn add_edge() {
        let mut g = Genome::new(1, 1, ignore);
        g.add_edge(0, 2, 1, 15.0, true);
        let input: Vec<f64> = vec![1.0; 1 as usize];
        let output = g.evaluate(&input);
        let expected = vec![15.0];
        assert_eq!(output, expected);
    }

    #[test]
    fn disable_edge() {
        let mut g = Genome::new(1, 1, ignore);
        g.add_edge(0, 2, 1, 15.0, true);
        g.disable_edge(0, 2);
        let input: Vec<f64> = vec![1.0; 1 as usize];
        let output = g.evaluate(&input);
        let expected = vec![0.0];
        assert_eq!(output, expected);
        let expected = vec![15.0];
        g.enable_edge(0, 2);
        let output = g.evaluate(&input);
        assert_eq!(output, expected);
    }

    #[test]
    fn extra_random() {
        let mut g = Genome::new(5, 5, ignore);
        g.connect_ends();
        g.random_edge();
        assert_eq!(g.num_connections, 30);
    }

    #[test]
    fn add_random_disabled() {
        let mut g = Genome::new(1, 1, ignore);
        g.connect_ends();
        g.disable_edge(0, 1);
        let e: (i32, i32) = g.random_edge();
        g.enable_edge(e.0, e.1);
        assert_eq!(g.nodes[0].borrow().adj.len(), 1);
    }

    #[test]
    fn unflatte_size() {
        let mut g = Genome::new(10, 10, ignore);
        g.connect_ends();
        let temp = g.flatten();
        assert_eq!(temp.len(), 10 * 10 + 10);
    }

    #[test]
    fn flatten_unflatten() {
        let mut g = Genome::new(10, 10, ignore);
        g.connect_ends();
        let input: Vec<f64> = vec![1.0; 10 as usize];
        let o1 = g.evaluate(&input);
        let temp = g.flatten();
        let g2 = Genome::un_flatten(&temp, 10, 10, ignore);
        let o2 = g2.evaluate(&input);
        assert_eq!(o1, o2);
    }

    #[test]
    fn random_flatten() {
        let mut g = Genome::new(10, 10, ignore);
        for _ in 0..5 {
            g.random_split();
        }
        for _ in 0..5 {
            g.random_edge();
        }
        let input: Vec<f64> = vec![1.0; 10 as usize];
        let o1 = g.evaluate(&input);
        let temp = g.flatten();
        let g2 = Genome::un_flatten(&temp, 10, 10, ignore);
        let o2 = g2.evaluate(&input);
        assert_eq!(o1, o2);
    }

    #[test]
    fn population_test() {
        let mut p = Population::new(5, 10, 10, ignore, false);
        for child in 0..5 {
            for i in 0..10 {
                for j in 0..10 {
                    p.population[child as usize].add_edge(i, j + 10 + 1, i * 10 + j, 1.0, true);
                }
            }
        }
        let input: Vec<f64> = vec![1.0; 10 as usize];
        let o: Vec<f64> = vec![10.0; 10 as usize];
        for i in 0..5 {
            let o1 = p.population[i].evaluate(&input);
            assert_eq!(o, o1);
        }
    }

    #[test]
    fn population_mutate() {
        fn metric(_inputs: &Vec<f64>, _outputs: &Vec<f64>) -> f64 {
            return 1.0;
        }
        let mut p1: Population = Population::new(1, 1, 1, ignore, true);
        let mut in1: Vec<f64> = vec![];
        in1.push(1.0);
        let mut outs = p1.evaluate_all(&in1, metric);
        p1.next_generation(&mut outs);
        assert!(p1.population[0].num_connections <= 2 || p1.population[0].num_nodes <= 3);
    }
}
