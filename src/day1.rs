pub fn calculate (data: String) -> usize {
    let mut sums: Vec<usize> = Vec::new();
    let mut current_sum: usize = 0;
    for numstr in data.lines() {
        if !(numstr == "") {
            current_sum += numstr.parse::<usize>().unwrap();
        } else {
            sums.push(current_sum);
            current_sum = 0;
        }
    }
    return *sums.iter().max().unwrap();
}

pub fn calculate2 (data: String) -> usize {
    let mut sums: Vec<usize> = Vec::new();
    let mut current_sum: usize = 0;
    for numstr in data.lines() {
        if !(numstr == "") {
            current_sum += numstr.parse::<usize>().unwrap();
        } else {
            sums.push(current_sum);
            current_sum = 0;
        }
    }
    sums.sort();
    sums.reverse();
    return sums[0..3].to_vec().into_iter().sum();
}