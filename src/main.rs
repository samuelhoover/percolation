use std::collections::{HashMap, HashSet};

use ::percolation::clusters::*;
use ::percolation::consts::*;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

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

#[inline]
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
            // Double check percolation occurs (it should be impossible for it not to happen!)
            println!("Did not percolate!");
        }
    }

    // average percolation threshold values across all runs
    let mut sum: f32 = 0.0;
    for &p in &results {
        sum += p;
    }
    let critical_p: f32 = sum / results.len() as f32;

    // print results
    let a: String = format!("Iterations:     {NUM_ITERS}");
    let b: String = format!("[NROWS, NCOLS]: [{NROWS}, {NCOLS}]");
    let c: String = format!("p*:             {critical_p:.3}");
    let out: String = [a, b, c].join("\n");
    println!("{out}");
}
