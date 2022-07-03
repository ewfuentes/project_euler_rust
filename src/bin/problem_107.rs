use itertools::Itertools;

fn parse_line(line: &str) -> Vec<f64> {
    return line
        .split(',')
        .map(|entry| {
            if entry == "-" {
                0.0
            } else {
                entry.parse().unwrap()
            }
        })
        .collect();
}

fn parse_graph(matrix: &str) -> nalgebra::DMatrix<f64> {
    let matrix_rows: Vec<Vec<f64>> = matrix.split_whitespace().map(parse_line).collect();
    let num_rows = matrix_rows.len();
    let num_cols = matrix_rows[0].len();
    return nalgebra::DMatrix::from_vec(
        num_rows,
        num_cols,
        matrix_rows.into_iter().flatten().collect(),
    );
}

fn count_edges(graph: &nalgebra::DMatrix<f64>) -> usize {
    return (0..graph.nrows())
        .map(|row_idx| {
            (row_idx..graph.ncols()).filter(move |&col_idx| graph[(row_idx, col_idx)] > 0.0)
        })
        .flatten()
        .count();
}

fn compute_network_weight(graph: &nalgebra::DMatrix<f64>) -> f64 {
    return (0..graph.nrows())
        .map(|row_idx| (row_idx..graph.ncols()).map(move |col_idx| graph[(row_idx, col_idx)]))
        .flatten()
        .sum();
}

fn is_fully_connected(graph: &nalgebra::DMatrix<f64>) -> bool {
    let adjacency_mat: nalgebra::DMatrix<f64> = graph
        .clone_owned()
        .map(|elem| if elem > 0.0 { 1.0 } else { 0.0 })
        + nalgebra::DMatrix::identity(graph.nrows(), graph.ncols());

    let path_counts = adjacency_mat.pow(graph.nrows().try_into().unwrap());
    return path_counts.row(0).iter().all(|&elem| return elem > 0.0);
}

fn minimize_network(graph: nalgebra::DMatrix<f64>) -> Option<nalgebra::DMatrix<f64>> {
    let min_num_edges = graph.nrows() - 1;
    let edge_count = count_edges(&graph);
    if !is_fully_connected(&graph) {
        return None;
    }
    if edge_count == min_num_edges {
        return Some(graph);
    }

    let sorted_edges = (0..graph.nrows())
        .cartesian_product(0..graph.ncols())
        .filter(|(row_idx, col_idx)| col_idx > row_idx && graph[(*row_idx, *col_idx)] > 0.0)
        .sorted_by(|&a, &b| graph[a].partial_cmp(&graph[b]).unwrap())
        .rev();

    for edge in sorted_edges {
        let mut updated_graph = graph.clone_owned();
        updated_graph[(edge)] = 0.0;
        updated_graph[(edge.1, edge.0)] = 0.0;
        let maybe_result = minimize_network(updated_graph);
        if maybe_result.is_some() {
            return maybe_result;
        }
    }
    return None;
}

fn main() {
    let file_name = std::path::Path::new("src/bin/p107_network.txt");
    let matrix = std::fs::read_to_string(file_name).unwrap();
    let matrix = parse_graph(&matrix);
    let starting_weight = compute_network_weight(&matrix);
    let minimal_graph = minimize_network(matrix).unwrap();
    let minimal_graph_weight = compute_network_weight(&minimal_graph);
    println!(
        "start: {} end: {} delta: {}",
        starting_weight,
        minimal_graph_weight,
        starting_weight - minimal_graph_weight
    );
}
