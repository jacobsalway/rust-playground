fn merge(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            res.push(left[i]);
            i = i + 1;
        } else {
            res.push(right[j]);
            j = j + 1;
        }
    }
    if i < left.len() {
        res.extend(left[i..].to_vec());
    }
    if j < right.len() {
        res.extend(right[j..].to_vec());
    }
    res
}

fn merge_sort(vec: &Vec<i32>) -> Vec<i32> {
    if vec.len() < 2 {
        vec.to_vec()
    } else {
        let midpoint = vec.len() / 2;
        let left = merge_sort(&vec[..midpoint].to_vec());
        let right = merge_sort(&vec[midpoint..].to_vec());
        merge(&left, &right)
    }
}

fn main() {
    let input = vec![11, 4, 8, 3, 2, 11, 23];
    print!("{:?}", merge_sort(&input));
}
