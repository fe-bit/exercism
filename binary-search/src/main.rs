fn binary_search(slice: &[i32], n: i32) -> bool {
    let middle_i = slice.len() / 2 as usize;
    let middle = slice[middle_i];
    if middle == n {
        println!("{:?} => {}", slice, middle);
        return true;
    }
    if slice.len() == 1 as usize {
        return false;
    }
    if middle > n {
        println!("middle > n => {:?}", slice);
        return binary_search(&slice[0..middle_i], n);
    } else {
        println!("middle < n => {:?}", slice);
        return binary_search(&slice[middle_i..], n);
    }
}

fn main() {
    assert!(binary_search(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 2));
    assert!(!binary_search(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 11));
    assert!(binary_search(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 8));
}
