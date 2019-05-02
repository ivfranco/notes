use permutohedron::Heap;

#[derive(Clone, Copy)]
pub enum Instr {
    A,
    B,
    C,
    Noop,
}

use Instr::*;

fn fit(instrs: &[Instr], interval: usize) -> bool {
    let mut a_uses = vec![0; interval];
    let mut b_uses = vec![0; interval];
    let mut c_uses = vec![0; interval];

    let mut accum = |i: usize, instr: Option<&Instr>| match instr {
        Some(A) => a_uses[i] += 1,
        Some(B) => b_uses[i] += 1,
        Some(C) => c_uses[i] += 1,
        _ => (),
    };

    for chunk in instrs.chunks(interval) {
        for i in 0..interval {
            accum(i, chunk.get(i));
        }
    }

    a_uses.into_iter().all(|v| v <= 1)
        && b_uses.into_iter().all(|v| v <= 1)
        && c_uses.into_iter().all(|v| v <= 1)
}

fn fit_in_one_delay(instrs: &[Instr], interval: usize) -> bool {
    (0..=instrs.len()).any(|i| {
        let mut extended = instrs.to_vec();
        extended.insert(i, Noop);
        fit(&extended, interval)
    })
}

pub fn report(instrs: &mut [Instr], interval: usize) -> (u32, u32) {
    let heap = Heap::new(instrs);
    let mut fit_0 = 0;
    let mut fit_1 = 0;

    for p in heap {
        if fit(&p, interval) {
            fit_0 += 1;
        } else if fit_in_one_delay(&p, interval) {
            fit_1 += 1;
        }
    }

    (fit_0, fit_1)
}
