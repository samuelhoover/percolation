use crate::consts::*;
use plotters::prelude::*;

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

pub fn plot_array(&arr: &[u32; LEN], perc_cluster: u32, name: &str) {
    let root = BitMapBackend::new(name, (NCOLS as u32, NROWS as u32)).into_drawing_area();

    let areas = root.split_evenly((NCOLS, NROWS));

    for (area, i) in areas.into_iter().zip(0..LEN) {
        if arr[i] == 0u32 {
            let _ = area.fill(&BLACK);
        } else if arr[i] == perc_cluster {
            let _ = area.fill(&RGBColor(166, 235, 153));
        } else {
            let _ = area.fill(&RGBColor(181, 215, 228));
        }
    }
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
