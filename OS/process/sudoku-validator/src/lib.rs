use crossbeam_utils::thread;

const SIZE: usize = 9;
const GRID_SIZE: usize = 3;

// assume sudoku solution is stored in row major
pub fn validate(sudoku: &[u8]) -> bool {
    assert!(sudoku.iter().all(|&slot| slot <= 9));
    assert_eq!(sudoku.len(), SIZE * SIZE);

    thread::scope(|s| {
        let mut validation = vec![];
        validation.push(s.spawn(|_| validate_row(sudoku)));
        validation.push(s.spawn(|_| validate_col(sudoku)));
        for top in (0..SIZE).step_by(GRID_SIZE) {
            for left in (0..SIZE).step_by(GRID_SIZE) {
                validation.push(s.spawn(move |_| validate_grid(sudoku, top, left)));
            }
        }

        validation.into_iter().all(|handle| handle.join().unwrap())
    })
    .unwrap()
}

fn validate_row(sudoku: &[u8]) -> bool {
    sudoku.chunks(SIZE).all(|row| one_to_nine(row))
}

fn validate_col(sudoku: &[u8]) -> bool {
    let mut slice = [0; SIZE];
    (0..SIZE).all(|col| {
        for row in 0..SIZE {
            slice[row] = sudoku[row * SIZE + col];
        }
        one_to_nine(&slice)
    })
}

fn validate_grid(sudoku: &[u8], top: usize, left: usize) -> bool {
    let mut slice = [0; SIZE];
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            slice[row * GRID_SIZE + col] = sudoku[(top + row) * SIZE + (left + col)];
        }
    }
    one_to_nine(&slice)
}

fn one_to_nine(slice: &[u8]) -> bool {
    let mut mask: u32 = 0;
    for slot in slice {
        mask |= 1u32 << slot;
    }
    mask == 0b11_1111_1110
}

#[test]
fn one_to_nine_test() {
    assert!(one_to_nine(&[1, 2, 3, 4, 5, 6, 7, 8, 9]));
    assert!(!one_to_nine(&[1, 2, 3, 4, 6, 6, 7, 8, 9]));
}

#[test]
fn validate_test() {
    let sudoku = [
        8, 6, 4, 3, 7, 1, 2, 5, 9, 3, 2, 5, 8, 4, 9, 7, 6, 1, 9, 7, 1, 2, 6, 5, 8, 4, 3, 4, 3, 6,
        1, 9, 2, 5, 8, 7, 1, 9, 8, 6, 5, 7, 4, 3, 2, 2, 5, 7, 4, 8, 3, 9, 1, 6, 6, 8, 9, 7, 3, 4,
        1, 2, 5, 7, 1, 3, 5, 2, 8, 6, 9, 4, 5, 4, 2, 9, 1, 6, 3, 7, 8,
    ];
    assert!(validate(&sudoku));

    let sudoku = [
        8, 6, 4, 3, 7, 1, 2, 5, 9, 3, 2, 5, 8, 4, 9, 7, 6, 1, 9, 7, 1, 2, 6, 5, 8, 4, 3, 4, 3, 6,
        1, 9, 2, 5, 8, 7, 1, 9, 8, 6, 5, 7, 4, 3, 2, 3, 5, 7, 4, 8, 3, 9, 1, 6, 6, 8, 9, 7, 3, 4,
        1, 2, 5, 7, 1, 3, 5, 2, 8, 6, 9, 4, 5, 4, 2, 9, 1, 6, 3, 7, 8,
    ];
    assert!(!validate(&sudoku));
}
