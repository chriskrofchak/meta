// EXAMPLES FOR PIPE AND COMP
mod pipe;

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

macro_rules! mult_assert {
    ($x:expr, $n:expr, $($f:expr),*) => {
        $(
            assert_eq!( $x, $f($n) );
            println!("Assert passed! {}({}) is {}.", stringify!($f), $n, $x);
        )*
    };
}

fn main() {

    // showing LTR and default pipe! behaves as LTR
    let nested_ltr = |n| square(double(succ(succ(n))));
    let ltr_piped_1 = ltr_pipe!(succ; succ; double; square);
    let piped_1 = pipe!(succ; succ; double; square);

    mult_assert!( 36, 1, nested_ltr, ltr_piped_1, piped_1 );

    // showing RTL 
    let nested_rtl = |n| succ(succ(double(square(n))));
    let rtl_piped_1 = rtl_pipe!(succ; succ; double; square);

    mult_assert!( 20, 3, nested_rtl, rtl_piped_1 );

    // just one final example where all 4 functions are different just so we guarantee
    // what is going on is what's going on 
    let triple = |n| k_n(3)(n);

    let final_clos = |n| square(triple(double(succ(n))));
    let rtl_final  = rtl_pipe!( square; triple; double; succ );
    let ltr_final  = ltr_pipe!( succ; double; triple; square );
    let pipe_fin   = pipe!( succ; double; triple; square );    

    mult_assert!( 144, 1, final_clos, rtl_final, ltr_final, pipe_fin );
}

// ALL ASSERTIONS PASS!! 