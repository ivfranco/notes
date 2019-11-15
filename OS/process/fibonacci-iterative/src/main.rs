use std::{
    env, mem, process,
    sync::{Arc, Condvar, Mutex},
    thread,
};

fn main() {
    let mut args = env::args();
    args.next();

    let n = args
        .next()
        .and_then(|arg| arg.parse::<u32>().ok())
        .unwrap_or_else(|| {
            eprintln!("Usage: EXEC N_FIONACCI");
            process::exit(1);
        });

    let tuple = Arc::new((
        Mutex::new(0),
        Mutex::new(false),
        Mutex::new(true),
        Condvar::new(),
        Condvar::new(),
    ));
    let (fib, parent_lock, child_lock, parent_cvar, child_cvar) = &*tuple;

    let tuple_clone = tuple.clone();
    thread::spawn(move || {
        fibonacci_calc(n, tuple_clone);
    });

    let mut ready = parent_lock.lock().unwrap();
    for _ in 0..n {
        while !*ready {
            ready = parent_cvar.wait(ready).unwrap();
        }
        print!("{}, ", *fib.lock().unwrap());
        {
            *ready = false;
            *child_lock.lock().unwrap() = true;
        }
        child_cvar.notify_one();
    }
}

type Tuple = Arc<(Mutex<u32>, Mutex<bool>, Mutex<bool>, Condvar, Condvar)>;

fn fibonacci_calc(n: u32, tuple: Tuple) {
    let (fib, parent_lock, child_lock, parent_cvar, child_cvar) = &*tuple;
    let (mut a, mut b) = (0, 1);
    let mut ready = child_lock.lock().unwrap();

    for i in 0..n {
        while !*ready {
            ready = child_cvar.wait(ready).unwrap();
        }
        {
            *fib.lock().unwrap() = b;
            *ready = false;
        }
        mem::swap(&mut a, &mut b);
        if let Some(sum) = b.checked_add(a) {
            b = sum;
        } else {
            eprintln!(
                "Error: overflowed u32 while calculating {}th fibonacci number",
                i + 1
            );
            process::exit(1);
        }

        {
            *parent_lock.lock().unwrap() = true;
        }
        parent_cvar.notify_one();
    }
}
