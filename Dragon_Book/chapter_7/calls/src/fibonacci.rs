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

pub fn fib0(n: u32) -> (u32, Activation) {
    let name = format!("fib0({})", n);
    if n >= 2 {
        let (a, tree) = fib1(n);
        (a, Activation::new(name, vec![tree]))
    } else {
        (1, Activation::new(name, vec![]))
    }
}

fn fib1(n: u32) -> (u32, Activation) {
    let name = format!("fib1({})", n);
    if n >= 4 {
        let (a, tree) = fib2(n);
        (a, Activation::new(name, vec![tree]))
    } else {
        let (a, f1) = fib0(n - 1);
        let (b, f2) = fib0(n - 2);
        (a + b, Activation::new(name, vec![f1, f2]))
    }
}

fn fib2(n: u32) -> (u32, Activation) {
    let name = format!("fib2({})", n);
    let (a, f1) = fib1(n - 1);
    let (b, f2) = fib1(n - 2);
    (a + b, Activation::new(name, vec![f1, f2]))
}

#[test]
fn fibonacci_test() {
    let (a, tree) = fibonacci(9);
    assert_eq!(a, 55);
    assert_eq!(tree.depth(), 9);

    let (a, tree) = fib0(9);
    assert_eq!(a, 55);
    assert_eq!(tree.depth(), 17);
}
