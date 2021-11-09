// comp for COMPOSITION
// basically composition x of y of ...
// #[macro_export]
macro_rules! comp {
    ($x:expr; $y:expr) => { |n| $x($y(n)) };
}

// #[macro_export]
macro_rules! pipe {
    // base cases
    () => {|n| n}; // identity function
    ($x:expr) => {|n| $x(n)}; // return x function on its own
    // recursive reversal case
    ($x:expr ; $($rest:expr);* ) => { comp!(pipe!($($rest);*) ; $x) }; // else |n| rest(x(n))
}