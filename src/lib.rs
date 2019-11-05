use std::collections::HashSet;
use std::u64;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut greatests: HashSet<(usize, usize)> = HashSet::new();
    let mut smallests: HashSet<(usize, usize)> = HashSet::new();
    let mut width = 0;

    for (x, row) in input.iter().enumerate() {
        if row.len() > width {
            width = row.len()
        };
        let mut greatest_so_far = &0u64;
        let mut greatest_in_row: Vec<(usize, usize)> = Vec::new();
        for (y, entry) in row.iter().enumerate() {
            if entry > greatest_so_far {
                greatest_so_far = entry;
                greatest_in_row.clear();
                greatest_in_row.push((x, y));
            } else if entry == greatest_so_far {
                greatest_in_row.push((x, y));
            }
        }

        for g in greatest_in_row {
            greatests.insert(g);
        }
    }

    for y in 0..width {
        let mut smallest_so_far = u64::max_value();
        let mut smallest_in_column: Vec<(usize, usize)> = Vec::new();
        for (x, row) in input.iter().enumerate() {
            if y >= row.len() {
                break;
            }
            let entry = row[y];
            if entry < smallest_so_far {
                smallest_so_far = entry;
                smallest_in_column.clear();
                smallest_in_column.push((x, y));
            }
            if entry == smallest_so_far {
                smallest_in_column.push((x, y));
            }
        }
        for s in smallest_in_column {
            smallests.insert(s);
        }
    }
    let saddle_points = greatests.intersection(&smallests);
    saddle_points.cloned().collect()
}
