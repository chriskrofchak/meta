// comp for COMPOSITION
// basically composition x of y of ...
#[macro_export]
macro_rules! comp {
    ($x:expr; $y:expr) => { |n| $x($y(n)) };
}

#[macro_export]
macro_rules! pipe {
    // base cases
    () => {|n| n}; // identity function
    ($x:expr) => {|n| $x(n)}; // return x function on its own
    (LTR $x:expr) => {|n| $x(n)};
    (RTL $x:expr) => {|n| $x(n)};
    // recursive reversal case
    ($x:expr ; $($rest:expr);* ) => { comp!(pipe!($($rest);*) ; $x) };
    (LTR $x:expr ; $($rest:expr);* ) => { comp!(pipe!(LTR $($rest);*) ; $x) };
    (RTL $x:expr ; $($rest:expr);* ) => { comp!($x ; pipe!(RTL $($rest);*)) }; 
}

// same as pipe! but just for no ambiguity also good to have ltr and rtl...
#[macro_export]
macro_rules! ltr_pipe {
    ($($x:tt)*) => {pipe!(LTR $($x)*)};
}

#[macro_export]
macro_rules! rtl_pipe {
    ($($x:tt)*) => {pipe!(RTL $($x)*)};
}