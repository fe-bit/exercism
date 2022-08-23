fn proverb(list: Vec<&str>) {
    for i in 0..list.len() {
        if i < list.len() - 1 {
            println!("For want of a {} the {} was lost.", list[i], list[i + 1]);
        } else {
            println!("And all for the want of a {}.", list[0]);
        }
    }
}

fn main() {
    proverb(vec![
        "nail", "shoe", "horse", "rider", "message", "battle", "kingdom",
    ]);
}
