#[derive(PartialEq, Eq, Debug)]
pub enum Comparison {
    Unequal,
    Equal,
    Sublist,
    Superlist,
}

fn sublist(list1: &[i32], list2: &[i32]) -> Comparison {
    let (shorter, longer) = if list1.len() > list2.len() {
        (list2, list1)
    } else {
        (list1, list2)
    };
    if sublist_rec(&shorter, &longer) {
        match list1.len() as i32 - list2.len() as i32 {
            0 => return Comparison::Equal,
            1.. => return Comparison::Superlist,
            std::i32::MIN..=-1 => return Comparison::Sublist,
        }
    } else {
        return Comparison::Unequal;
    }
}

fn sublist_rec(shorter: &[i32], longer: &[i32]) -> bool {
    if shorter.len() == 0 || longer.len() == 0 || shorter.len() > longer.len() {
        return false;
    }

    if &longer[0..shorter.len()] == shorter {
        return true;
    } else {
        return sublist_rec(shorter, &longer[1..]);
    }
}

fn main() {
    assert_eq!(sublist(&[1, 2, 3], &[1, 2, 3, 4, 5]), Comparison::Sublist);
    assert_eq!(
        sublist(&[1, 2, 3, 4, 5], &[1, 2, 3, 4, 5]),
        Comparison::Equal
    );
    assert_eq!(
        sublist(&[1, 2, 3, 4, 5, 6], &[1, 2, 3]),
        Comparison::Superlist
    );
    assert_eq!(sublist(&[1, 2, 2], &[1, 2, 3, 4, 5]), Comparison::Unequal);
}
