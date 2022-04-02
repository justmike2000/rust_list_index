fn main() {
    // Print out each item of each list in a new list ordered by index of original list
    let our_list = vec![vec![2, 4, 5], vec![3, 9], vec![], vec![2], vec![0, 3]];
    let mut temp = vec![];
    let (mut max_count, mut pos_index) = (1, 0);

    while pos_index != max_count {
        for sub_list in our_list.iter() {
            // Recalculate max count
            if sub_list.len() > max_count {
                max_count = sub_list.len();
            } 
            // If can pop that sucker in!
            if pos_index < sub_list.len() {
                 temp.push(sub_list[pos_index]);
            }
        }
        // acc+1
        pos_index += 1;
    }
    println!("original: {:?}", our_list);
    println!("new: {:?}", temp);
}
