fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vec1 yields `&i32`
    let mut iter = vec1.iter();
    // `into_iter()` for vec2 yields `i32`
    let mut into_iter = vec2.into_iter();

    // `iter()` yields a reference and we want to reference one of its
    // items, so we have to destructure `&&i32` to `i32`
    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    // `into_iter()` yields a value, and we want to reference one of its
    // items, so we have to destructure `&i32` to `i32`
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    println!(
        "Find 2 in array2: {:?}",
        array2.into_iter().find(|&x| x == 2)
    );

    let vec = vec![1, 9, 3, 3, 13, 2];
    // `iter()` for vecs yields a reference and `position()` does not take a
    // reference, we have to destructure `&i32` to `i32`
    let index_of_first_even_number = vec.iter().position(|&x| x % 2 == 0);
    assert_eq!(index_of_first_even_number, Some(5));

    let index_of_first_negative_number = vec.into_iter().position(|x| x < 0);
    assert_eq!(index_of_first_negative_number, None);
}
