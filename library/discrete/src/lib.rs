use itertools::Itertools;
use std::{vec, collections::HashSet};

pub fn perm_ascending_and_without_repetitions(len: usize, len_res: usize) -> Vec<Vec<u8>> {
    let v: Vec<u8> = (0..(len as u8)).collect();
    let mut sorted_v = v.clone();
    sorted_v.sort();
    let perms = sorted_v.into_iter().permutations(len_res);

    let mut res = Vec::new();

    for perm in perms {
        if is_sorted(&perm) {
            res.push(perm);
        }
    }

    res
}

fn is_sorted(v: &[u8]) -> bool {
    let mut prev = u8::min_value();
    for &x in v {
        if x < prev {
            return false;
        }
        prev = x;
    }
    true
}