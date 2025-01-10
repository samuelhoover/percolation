use crate::consts::*;

pub fn get_neigbors(&i: &usize) -> (Option<usize>, Option<usize>, Option<usize>, Option<usize>) {
    // Get neighbors in lattice

    // if index is in first row -> no "up" neighbor
    // if index is in first column -> no "left" neighbor
    // if index in is last row -> no "down" neighbor
    // if index is in last column -> no "right" neighbor
    let up: Option<usize> = if i < NCOLS { None } else { Some(i - NCOLS) };
    let down: Option<usize> = if i >= LEN - NCOLS - 1 {
        None
    } else {
        Some(i + NCOLS)
    };
    let left: Option<usize> = if i % NCOLS == 0 { None } else { Some(i - 1) };
    let right: Option<usize> = if (i + 1) % NCOLS == 0 {
        None
    } else {
        Some(i + 1)
    };

    (up, down, left, right)
}

pub fn print_array(arr: &[u32; LEN]) {
    for (i, elem) in arr.iter().enumerate() {
        if (i + 1) % NCOLS == 0 {
            println!("{:0>3}", elem);
        } else {
            print!("{:0>3} ", elem);
        }
    }
}
