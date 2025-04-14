use std::collections::{linked_list, HashMap, HashSet};

fn main() {
    // sequence where each element is a unique summation of the previous elements (also greater than the previous elements)
    let mut sequence: Vec<u32> = vec![1, 2];

    for _ in 0..10 {
        println!("{}", generate(&mut sequence));
    }
}

fn next(arr: &[u32]) -> u32 {
    let len = arr.len();

    // key pairs are SORTED 
    let mut values = HashMap::<u32, u32>::new();

    for i in 0..len {
        for k in 0..len {
            // indices cannot be the same
            if i == k {
                continue;
            }

            // must sum up to at least the max element (last element by proof by induction?)
            let max = arr[len-1];

            let a_i = arr[i];
            let a_k = arr[k];

            if (a_i + a_k) > max {
                values.entry(a_i + a_k).and_modify(|x| *x += 1).or_default();
            }
        }    
    }

    // dedupping step
    values.retain(|k, v| *v == 1);

    // minimizing step
    let max = values.keys().min().unwrap();
    *max
}

fn generate(vec: &mut Vec<u32>) -> u32 {
    let new = next(&vec);
    vec.push(new);
    new
}