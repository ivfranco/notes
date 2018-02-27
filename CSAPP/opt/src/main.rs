fn main() {
    let v: Vec<data_t> = (1..1000).map(|u| u as data_t).collect();
    let mut dest = IDENT;
    println!("{}", dest);
}

type data_t = i32;
const IDENT: data_t = 1;
#[inline(always)]
fn op(a: data_t, b: data_t) -> data_t {
    a * b
}

fn combine5(v: &[data_t], dest: &mut data_t) {
    let mut i = 0;
    let length = v.len() as isize;
    let limit = length - 4;
    let data: *const data_t = v.as_ptr();
    let mut acc = IDENT;

    unsafe {
        while i < limit {
            acc = op(acc, *data.offset(i));
            acc = op(acc, *data.offset(i + 1));
            acc = op(acc, *data.offset(i + 2));
            acc = op(acc, *data.offset(i + 3));
            acc = op(acc, *data.offset(i + 4));
            i += 5;
        }

        while i < length {
            acc = op(acc, *data.offset(i));
            i += 1;
        }
    }

    *dest = acc;
}

fn merge(src1: &mut [i32], src2: &mut [i32], dest: &mut [i32], n: usize) {
    let mut i1 = 0;
    let mut i2 = 0;
    let mut id = 0;

    while i1 < n && i2 < n {
        let (min, i1_inc, i2_inc) = if src1[i1] < src2[i2] {
            (src1[i1], 1, 0)
        } else {
            (src2[i2], 0, 1)
        };
        dest[id] = min;
        i1 += i1_inc;
        i2 += i2_inc;
        id += 1;
    }

    while i1 < n {
        dest[id] = src1[i1];
        id += 1;
        i1 += 1;
    }
    while i2 < n {
        dest[id] = src2[i2];
        id += 1;
        i2 += 1;
    }
}

fn psum1(a: &[f32], p: &mut [f32], n: usize) {
    let mut i = 1;
    let mut acc = a[0];
    p[0] = acc;
    while i < n {
        acc += a[i];
        p[i] = acc;
        i += 1;
    }
}
