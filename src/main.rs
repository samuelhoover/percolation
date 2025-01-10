use std::collections::{HashMap, HashSet};

use ::percolation::consts::*;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

fn count_clusters(arr: &mut [u32; LEN]) -> HashMap<u32, u32> {
    let mut clusters: HashMap<u32, u32> = HashMap::new();

    for elem in arr.iter() {
        if *elem > 0 {
            *clusters.entry(*elem).or_insert(0) += 1;
        }
    }

    clusters
}

fn check_percolation(arr: &[u32; LEN], clusters: &HashMap<u32, u32>) -> bool {
    let top_row: HashSet<_> = arr[0..NCOLS].iter().cloned().collect();
    let bottom_row: HashSet<_> = arr[(LEN - NCOLS)..LEN].iter().cloned().collect();
    let left_column: HashSet<_> = arr[0..(LEN - NCOLS)].iter().step_by(NCOLS).collect();
    let right_column: HashSet<_> = arr[(NCOLS - 1)..(LEN - NCOLS)]
        .iter()
        .step_by(NCOLS)
        .collect();

    let mut percolated: bool = false;
    for (&cluster_id, _) in clusters.iter() {
        let touches_top = top_row.contains(&cluster_id);
        let touches_bottom = bottom_row.contains(&cluster_id);
        let touches_left = left_column.contains(&cluster_id);
        let touches_right = right_column.contains(&cluster_id);

        let percolates_vertically = touches_top && touches_bottom;
        let percolates_horizontally = touches_left && touches_right;

        if percolates_vertically {
            // println!("Cluster {:0>3} percolates vertically", cluster_id);
            percolated = true;
            break;
        } else if percolates_horizontally {
            // println!("Cluster {:0>3} percolates horizontally", cluster_id);
            percolated = true;
            break;
        }
    }

    percolated
}

fn print_array(arr: &[u32; LEN]) {
    for (i, elem) in arr.iter().enumerate() {
        if (i + 1) % NCOLS == 0 {
            println!("{:0>3}", elem);
        } else {
            print!("{:0>3} ", elem);
        }
    }
}

fn get_neigbors(&i: &usize) -> (Option<usize>, Option<usize>, Option<usize>, Option<usize>) {
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

fn clustering(arr: &mut [u32; LEN]) {
    let mut change: u32 = 0;
    for _ in 0..LEN {
        for i in 0..LEN {
            if arr[i] > 0 {
                let (up, down, left, right) = get_neigbors(&i);

                if let Some(left) = left {
                    if arr[left] > arr[i] {
                        arr[i] = arr[left];
                        change += 1;
                    }
                }
                if let Some(right) = right {
                    if arr[right] > arr[i] {
                        arr[i] = arr[right];
                        change += 1;
                    }
                }
                if let Some(down) = down {
                    if arr[down] > arr[i] {
                        arr[i] = arr[down];
                        change += 1;
                    }
                }
                if let Some(up) = up {
                    if arr[up] > arr[i] {
                        arr[i] = arr[up];
                        change += 1;
                    }
                }
            }
        }

        if change > 0 {
            change = 0;
        } else {
            // break early if clusters did not change
            break;
        }
    }
}

fn main() {
    // define PRNG
    let mut rng: SmallRng = SmallRng::from_entropy();

    // store all percolation threshold values
    let mut results: [f32; NUM_ITERS as usize] = [0f32; NUM_ITERS as usize];

    for i in 0..NUM_ITERS {
        // create fully blocked lattice
        let mut percolate: bool = false;
        let mut arr: [u32; LEN] = [0u32; LEN];

        for j in 1..=LEN {
            // randomly pick blocked site to open
            let mut open_site: bool = false;
            let mut site: usize = rng.gen_range(0..LEN);
            while !open_site {
                if arr[site] == 0 {
                    open_site = true
                } else {
                    site = rng.gen_range(0..LEN);
                }
            }

            // assign newly opened site a new number
            arr[site] = j as u32;

            // create clusters
            clustering(&mut arr);
            let clusters: HashMap<u32, u32> = count_clusters(&mut arr);

            percolate = check_percolation(&arr, &clusters);
            if percolate {
                // print_array(&arr);
                results[i as usize] = (j as f32) / (LEN as f32);
                break;
            }
        }
        if !percolate {
            println!("Did not percolate!");
        }
    }

    let mut sum: f32 = 0.0;
    for &p in &results {
        sum += p;
    }
    let critical_p: f32 = sum / results.len() as f32;
    let a: String = format!("Iterations:     {NUM_ITERS}");
    let b: String = format!("[NROWS, NCOLS]: [{NROWS}, {NCOLS}]");
    let c: String = format!("p*:             {critical_p:.3}");
    let out: String = [a, b, c].join("\n");
    println!("{out}");
}
