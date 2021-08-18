use std::collections::HashSet;

pub fn find_two_number_sum_with_hash_map (target: i32, integer_vector: &Vec<i32>) -> Option<(i32, i32)> {
    let mut was_traversed = HashSet::new();

    for x in integer_vector {
        let y = target - x;

        if was_traversed.contains(&y) {
            return Some((*x, y));
        } else {
            was_traversed.insert(x);
        }
    }

    None
}

#[cfg(test)]

mod test {
    use super::*;
    // testing find_two_number_sum_with_hash_map
    #[test]
    fn when_given_a_target_and_a_vector_of_ints_one() {
        let inputs = [
            (9, vec![-3, -6, -44, 3, 9, 44, 6, 4]),
            (5, vec![3, 2, 6, -16, 0, 100])
        ];

        for (target, integers) in inputs.iter() {
            match find_two_number_sum_with_hash_map(*target, integers) {
                Some((x, y)) => assert_eq!(x + y, *target),
                _ => () 
            }
        }
    }
}