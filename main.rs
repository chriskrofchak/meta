// EXAMPLES FOR PIPE AND COMP

mod pipe;
use pipe::pipe;

fn k_n(k: u32) -> Box<dyn Fn(u32) -> u32> {
    Box::new(move |x| k*x)
}

fn double(n: u32) -> u32 {
    k_n(2)(n)
}

fn succ(n: u32) -> u32 {
    n+1
}

fn square(n: u32) -> u32 {
    n*n
}

fn main() {
    let nested_1 = |n| square(double(succ(succ(n))));
    let piped_1 = pipe!(succ; succ; double; square);

    assert_eq!(nested_1(1), 36);
    assert_eq!(piped_1(1), 36);

    println!("{} vs. {}", nested_1(1), piped_1(1));
}