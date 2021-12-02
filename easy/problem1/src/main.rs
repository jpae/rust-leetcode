use std::collections::HashMap;

fn two_sum(nums: &Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash = HashMap::<i32, usize>::new();

    for x in 0..nums.len() {
        let expected = target - nums[x];
        match hash.get(&expected) {
            Some(&found) => return vec![found as i32, x as i32],
            None => hash.insert(nums[x], x),
        };
    }

    // for x in 0..nums.len() {
    //     let expected = target - nums[x];
    //     if let Some(&found) = hash.get(&expected) {
    //         return vec![found as i32, x as i32]
    //     } else {
    //         hash.insert(nums[x], x);
    //     }
    // }

    unreachable!()
}

fn main() {
    let vec = vec![2, 7, 11, 15];
    assert_eq!(two_sum(&vec, 9), vec![0, 1]);
    assert_eq!(two_sum(&vec, 17), vec![0, 3]);

    let vec = vec![3, 3, 6, 5];
    assert_eq!(two_sum(&vec, 6), vec![0, 1]);

    let vec = vec![3, 2, 4];
    assert_eq!(two_sum(&vec, 6), vec![1, 2]);
}
