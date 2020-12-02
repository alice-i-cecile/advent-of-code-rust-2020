use std::{cmp::Ordering, path::Path};
use std::error::Error;

use aoc_lib::read_integers;
fn main() -> Result<(), Box<dyn Error>> {
    let input = read_integers(Path::new("day-1/input.txt"))?;
    if let Some(prod) = find_2020_product(input) {
        println!("{}", prod);
    } else {
        println!("no pair sum to 2020");
    }
    Ok(())
}

fn find_2020_product(mut v: Vec<isize>) -> Option<isize> {
    v.sort();
    let mut it = v.iter();
    let (mut left, mut right) = (it.next(), it.next_back());
    while let (Some(n_left), Some(n_right)) = (left.cloned(), right.cloned()) {
        match (n_left + n_right).cmp(&2020) {
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

    macro_rules! test_cases {
        ($($name:ident: $data:expr => $want:expr),*$(,)?) => {$(
            #[test]
            fn $name(){
                let data: Vec<isize> = $data;

                assert_eq!(find_2020_product(data), $want);
            }
        )*};
    }
    test_cases! {
        empty: vec![] => None,
        one: vec![2020] => None,
        easy: vec![0, 2020] => Some(0),
        easy_reversed: vec![2020, 0] => Some(0),
        half: vec![1010, 1010] => Some(1010*1010),
        single_half: vec![0, 1010, 1] => None,
        middle: vec![1, 2, 3, 4, 5, 6, 7, 500, 666, 710, 1520, 2000, 2001, 34] => Some(500*1520),
    }
}
