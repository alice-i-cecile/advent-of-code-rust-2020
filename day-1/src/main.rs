use std::collections::HashSet;
use std::error::Error;
use std::{cmp::Ordering, path::Path};

use aoc_lib::read_integers;

// TODO: toggle between challenges based on feature flag?
fn main() -> Result<(), Box<dyn Error>> {
    let input = read_integers(Path::new("day-1/input.txt"))?;
    if let Some(prod) = find_product_hash_set(input, 2020) {
        println!("{}", prod);
    } else {
        println!("no pair sum to 2020");
    }
    Ok(())
}

// Runs in O(n) time
fn find_product_hash_set(v: Vec<isize>, target: isize) -> Option<isize> {
    let mut set: HashSet<isize> = HashSet::new();
    for i in v {
        let complement = target - i;
        if set.contains(&complement) {
            return Some(i * complement);
        } else {
            set.insert(i);
        }
    }
    None
}

// Solution for part 2
fn find_product_hash_set_3(v: Vec<isize>, target: isize) -> Option<isize> {
    // Perform the search in one function, rather than calling previous solution
    // to avoid rebuilding hash set repeatedly
    let mut set: HashSet<isize> = HashSet::new();
    for a in v {
        set.insert(a);

        let new_target = target - a;

        // TODO: can we remove this clone?
        for b in set.clone() {
            let complement = new_target - b;
            if set.contains(&complement) {
                return Some(a * b * complement);
            }
        }
    }
    None
}

// Runs in O(n log n) time because of the sort
#[allow(dead_code)]
fn find_product_walker(mut v: Vec<isize>, target: isize) -> Option<isize> {
    v.sort();
    let mut it = v.iter();
    let (mut left, mut right) = (it.next(), it.next_back());
    while let (Some(n_left), Some(n_right)) = (left.cloned(), right.cloned()) {
        match (n_left + n_right).cmp(&target) {
            Ordering::Less => left = it.next(),
            Ordering::Greater => right = it.next_back(),
            Ordering::Equal => return Some(n_left * n_right),
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[allow(unused_macros)]
    macro_rules! walker_2 {
        ($($name:ident: $data:expr => $want:expr),*$(,)?) => {$(
            #[test]
            fn $name(){
                let data: Vec<isize> = $data;

                assert_eq!(find_product_walker(data, 2020), $want);
            }
        )*};
    }

    macro_rules! hash_set_2 {
        ($($name:ident: $data:expr => $want:expr),*$(,)?) => {$(
            #[test]
            fn $name(){
                let data: Vec<isize> = $data;

                assert_eq!(find_product_hash_set(data, 2020), $want);
            }
        )*};
    }

    // TODO: how do we avoid namespace conflicts?
    /*
    walker_2! {
         empty: vec![] => None,
         one: vec![2020] => None,
         easy: vec![0, 2020] => Some(0),
         easy_reversed: vec![2020, 0] => Some(0),
         half: vec![1010, 1010] => Some(1010*1010),
         single_half: vec![0, 1010, 1] => None,
         middle: vec![1, 2, 3, 4, 5, 6, 7, 500, 666, 710, 1520, 2000, 2001, 34] => Some(500*1520),
     }
     */
    hash_set_2! {
        empty: vec![] => None,
        one: vec![2020] => None,
        easy: vec![0, 2020] => Some(0),
        easy_reversed: vec![2020, 0] => Some(0),
        half: vec![1010, 1010] => Some(1010*1010),
        single_half: vec![0, 1010, 1] => None,
        middle: vec![1, 2, 3, 4, 5, 6, 7, 500, 666, 710, 1520, 2000, 2001, 34] => Some(500*1520),
    }
}
