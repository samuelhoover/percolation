use crate::consts::LEN;
use crate::utils::get_neigbors;
use std::collections::HashMap;

pub fn count_clusters(arr: &mut [u32; LEN]) -> HashMap<u32, u32> {
    let mut clusters: HashMap<u32, u32> = HashMap::new();

    for elem in arr.iter() {
        if *elem > 0 {
            *clusters.entry(*elem).or_insert(0) += 1;
        }
    }

    clusters
}

// TODO: optimize double for loop
pub fn clustering(arr: &mut [u32; LEN]) {
    // Make clusters
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
