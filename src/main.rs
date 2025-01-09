use std::collections::{HashMap, HashSet};

use ::percolation::consts::*;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

fn count_clusters(arr: &mut [u32; LEN]) -> HashMap<u32, u32> {
    let mut clusters: HashMap<u32, u32> = HashMap::new();

    for i in 0..LEN {
        if arr[i] > 0 {
            *clusters.entry(arr[i]).or_insert(0) += 1;
        }
    }

    clusters
}

fn sort_clusters(clusters: &HashMap<u32, u32>) -> Vec<(u32, u32)> {
    let mut sorted_clusters: Vec<_> = clusters.iter().collect();
    sorted_clusters.sort_by(|a, b| b.1.cmp(a.1));
    sorted_clusters
        .into_iter()
        .map(|(&key, &value)| (key, value))
        .collect()
}

fn check_percolation(arr: &[u32; LEN], clusters: &HashMap<u32, u32>) {
    let top_row: HashSet<_> = arr[(NCOLS + 1)..(2 * NCOLS)].iter().cloned().collect();
    let bottom_row: HashSet<_> = arr[((NROWS - 2) * (NCOLS + 1))..((NROWS - 1) * NCOLS)]
        .iter()
        .cloned()
        .collect();
    let left_column: HashSet<_> = arr[NCOLS..(LEN - NCOLS)].iter().step_by(NCOLS).collect();
    let right_column: HashSet<_> = arr[NCOLS..(LEN - NCOLS)].iter().step_by(NCOLS).collect();

    for (&cluster_id, _) in clusters.iter() {
        let touches_top = top_row.contains(&cluster_id);
        let touches_bottom = bottom_row.contains(&cluster_id);
        let touches_left = left_column.contains(&cluster_id);
        let touches_right = right_column.contains(&cluster_id);

        let percolates_vertically = touches_top && touches_bottom;
        let percolates_horizontally = touches_left && touches_right;

        if percolates_vertically && percolates_horizontally {
            println!(
                "Cluster {:0>3} percolates both vertically and horizontally",
                cluster_id
            );
        } else if percolates_vertically {
            println!("Cluster {:0>3} percolates vertically", cluster_id);
        } else if percolates_horizontally {
            println!("Cluster {:0>3} percolates horizontally", cluster_id);
        }
    }
}

fn print_array(arr: &[u32; LEN]) {
    for i in 0..LEN {
        if (i + 1) % NCOLS == 0 {
            print!("{:0>3}\n", arr[i]);
        } else {
            print!("{:0>3} ", arr[i]);
        }
    }
    println!("");
}

fn clustering(arr: &mut [u32; LEN]) -> u32 {
    let mut change: u32 = 0;
    let mut loops: u32 = 0;
    let mut n: usize = 0;
    while n < LEN {
        for i in NCOLS..(LEN - NCOLS) {
            if arr[i] > 0 {
                if arr[i - 1] > arr[i] {
                    arr[i] = arr[i - 1];
                    change += 1;
                }
                if arr[i + 1] > arr[i] {
                    arr[i] = arr[i + 1];
                    change += 1;
                }
                if arr[i - NCOLS] > arr[i] {
                    arr[i] = arr[i - NCOLS];
                    change += 1;
                }
                if arr[i + NCOLS] > arr[i] {
                    arr[i] = arr[i + NCOLS];
                    change += 1;
                }
            }
        }

        loops += 1;
        if change > 0 {
            n += 1;
            change = 0;
        } else {
            n = LEN;
        }
    }

    loops
}

fn main() {
    // TODO: start with fully blocked array, randomly open sites until percolation occurs

    // define PRNG
    let mut rng: SmallRng = SmallRng::from_entropy();

    // create fully blocked lattice
    let mut arr: [u32; LEN] = [0u32; LEN];

    //populate array with consecutive numbers where 0 is rock and give it a rock boarder
    for i in NCOLS..(LEN - NCOLS) {
        if (i + 1) % NCOLS != 1 && (i + 1) % NCOLS != 0 && i % 5 > 0 {
            arr[i] = i as u32;
        }
    }
    print_array(&arr);

    // create clusters
    let loops: u32 = clustering(&mut arr);
    print_array(&arr);

    let clusters: HashMap<u32, u32> = count_clusters(&mut arr);
    let sorted_clusters: Vec<(u32, u32)> = sort_clusters(&clusters);

    for (cluster, size) in sorted_clusters.iter() {
        println!("Cluster {:0>3} size: {}", cluster, size);
    }

    println!("\nTotal cluster: {}\n", clusters.len());

    check_percolation(&arr, &clusters);

    println!("\nPercolation completed in {loops} loops (Max loops {LEN})");
}
