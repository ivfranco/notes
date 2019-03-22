use crate::Activation;

const PIVOT: usize = 0;

// start dooes not affect the sorting logic, only changes the function names in the generated activation tree
fn partition(start: usize, slice: &mut [u32]) -> (usize, Activation) {
    assert!(!slice.len() > 1);

    let pivot = slice[PIVOT];
    slice.swap(PIVOT, slice.len() - 1);
    let mut i = 0;

    for j in 0..slice.len() - 1 {
        if slice[j] <= pivot {
            slice.swap(i, j);
            i += 1;
        }
    }

    slice.swap(i, slice.len() - 1);

    let name = format!("p({},{})", start, start + slice.len());
    let activation = Activation::new(name, vec![]);

    (i, activation)
}

pub fn quicksort(start: usize, slice: &mut [u32]) -> Activation {
    let name = format!("q({},{})", start, start + slice.len());

    if slice.len() > 1 {
        let (i, p) = partition(start, slice);
        let q1 = quicksort(start, &mut slice[..i]);
        let q2 = quicksort(start + i + 1, &mut slice[i + 1..]);

        Activation::new(name, vec![p, q1, q2])
    } else {
        Activation::new(name, vec![])
    }
}

#[cfg(test)]
fn is_sorted(slice: &[u32]) -> bool {
    slice.iter().zip(slice.iter().skip(1)).all(|(a, b)| a <= b)
}

#[test]
fn quicksort_activation_tree() {
    let slice = &mut [2, 8, 7, 1, 3, 5, 6, 4];
    let tree = quicksort(0, slice);
    // println!("{:?}", tree);
    assert!(is_sorted(slice));
    assert_eq!(tree.depth(), 5);
}
