pub fn general_ershov(children: &[usize]) -> usize {
    assert!(!children.is_empty(), "Error: At least one child");

    let mut sorted = children.to_vec();
    sorted.sort_by(|a, b| a.cmp(b).reverse());
    sorted
        .into_iter()
        .enumerate()
        .map(|(a, b)| a + b)
        .max()
        .unwrap()
}

#[test]
fn ershov_number_test() {
    assert_eq!(general_ershov(&[1, 2, 3]), 3);
    assert_eq!(general_ershov(&[3, 3, 3]), 5);
}
