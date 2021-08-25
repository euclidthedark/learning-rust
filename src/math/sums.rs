use std::collections::HashSet;

pub fn find_two_number_sum_with_hash_map (target: i32, integer_vector: &Vec<i32>) -> Option<(i32, i32)> {
    let mut was_traversed = HashSet::new();

    if integer_vector[0] + integer_vector[1] == target {
        return Some((integer_vector[0], integer_vector[1]))
    }

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

fn find_two_number_sum_with_pointers(target: i32, integer_vector: &mut Vec<i32>) -> Option<(i32, i32)> {
    let mut left_pointer = 0;
    let mut right_pointer = integer_vector.len() - 1;
    integer_vector.sort();

    while left_pointer < right_pointer {
        let check_sum = integer_vector[left_pointer] + integer_vector[right_pointer]; 

        if check_sum == target {
            return Some((integer_vector[left_pointer], integer_vector[right_pointer]))
        } else if check_sum < target {
            left_pointer += 1;
        } else if check_sum > target {
            right_pointer -= 1;
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
                _ => assert_eq!(true, false) 
            }
        }
    }
    // testing two number sum using pointers
    #[test]
    fn when_given_a_target_and_a_vector_of_ints_two () {
        let inputs = [
            (9, vec![-3, -6, -44, 3, 9, 44, 6, 4]),
            (5, vec![3, 2, 6, -16, 0, 100])
        ];

        for (target, integers) in inputs.iter() {
            let mut mutable_integers = integers.clone();
            match find_two_number_sum_with_pointers(*target, &mut mutable_integers) {
                Some((x, y)) => assert_eq!(x + y, *target),
                _ => assert_eq!(true, false) 
            }
        }
    }
}
