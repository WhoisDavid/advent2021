#[allow(unused_macros)]
#[cfg(debug_assertions)]
macro_rules! debug {
    ($x:expr) => {
        dbg!($x)
    };
}

#[allow(unused_macros)]
#[cfg(not(debug_assertions))]
macro_rules! debug {
    ($x:expr) => {
        std::convert::identity($x)
    };
}

#[allow(unused)]
pub fn print_grid<T: Copy + std::fmt::Display>(t: &[T], s: usize) {
    println!();
    for (i, v) in t.iter().enumerate() {
        if i % s == 0 {
            println!()
        }
        print!("{:<4}", v)
    }
    println!();
}
