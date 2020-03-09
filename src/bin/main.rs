use try_rust::binary_tree::*;

fn main() {
    let mut first = insert(Box::new(None),7);
    first = insert(first, 2);
    first = insert(first, 1);
    first = insert(first, 5);
    first = insert(first, 4);
    first = insert(first, 15);
    first = insert(first, 10);
    first = insert(first, 8);
    //first = insert(first, 11);
    //first = insert(first, 12);
    first = insert(first, 14);
    first = insert(first, 13);
    first = insert(first, 17);
    first = insert(first, 16);
    first = insert(first, 19);
    first = remove(first, 15);

    //assert_eq!(true, find(&first, 15));
    assert_eq!(false, find(&first, 15));
    assert_eq!(true, find(&first, 14));
    assert_eq!(true, find(&first, 13));

    let right = get_right_node(&first);
    let right_left = get_left_node(&right);
    let right_left_right = get_right_node(&right_left);

    println!("right no {}", right.unwrap().get_no());
    println!("right left no {}", right_left.unwrap().get_no());
    println!("{:?}", right_left_right);
    println!("test github actions");

}
