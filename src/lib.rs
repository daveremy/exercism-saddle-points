pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points: Vec<(usize, usize)> = Vec::new();

    for (y, row) in input.iter().enumerate() {
        if let Some(max) = row.iter().max() {
            for (x, value) in row.iter().enumerate().filter(|(_, c)| c == &max) {
                if input
                    .iter()
                    .enumerate()
                    .map(|(_, r)| r[x])
                    .all(|v| &v >= value)
                {
                    saddle_points.push((y, x));
                }
            }
        }
    }

    saddle_points
}
