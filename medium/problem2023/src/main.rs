fn num_of_pairs(nums: Vec<String>, target: String) -> i32 {
    let mut total = 0;
    for (outndx, outval) in nums.iter().enumerate() {
        for (inndx, inval) in nums.iter().enumerate() {
            if inndx == outndx || outval.len() + inval.len() != target.len() {
                continue;
            } else if format!("{}{}", outval, inval).eq(&target) {
                total = total + 1;
            }
        }
    }
    total

    // TODO: Use a HashMap to optimize solution
}

fn main() {
    let arg = ["777","7","77","77"].iter().map(|&s| s.into()).collect();
    assert_eq!(num_of_pairs(arg, "7777".to_string()), 4);

    let arg = ["123","4","12","34"].iter().map(|&s| s.into()).collect();
    assert_eq!(num_of_pairs(arg, "1234".to_string()), 2);

    let arg = ["1","1","1"].iter().map(|&s| s.into()).collect();
    assert_eq!(num_of_pairs(arg, "11".to_string()), 6);
}
