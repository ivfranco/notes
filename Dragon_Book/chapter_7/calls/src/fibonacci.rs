use crate::Activation;

pub fn fibonacci(n: u32) -> (u32, Activation) {
    let name = format!("f({})", n);
    if n < 2 {
        (1, Activation::new(name, vec![]))
    } else {
        let (a, f1) = fibonacci(n - 1);
        let (b, f2) = fibonacci(n - 2);
        let tree = Activation::new(name, vec![f1, f2]);
        (a + b, tree)
    }
}

#[test]
fn fibonacci_test() {
    let (a, tree) = fibonacci(9);
    assert_eq!(a, 55);
    assert_eq!(tree.depth(), 9);
}
