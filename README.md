# meta

So pipe is a macro that I made because I wanted to emulate the pipe tool in Python which cleverly leverages `__or__` syntax with the pipe '|' such that a sequence of functions `f1,...,fn`. That you want applied in succession on some element we'll say `x`, we can write as follows:

```fn(...(f1(x))...)```

becomes

``` f1 | ... | fn ```

so in the case of my macro, we have 

```rust
pipe!(f1; ... ; fn);
```

will emulate pipe. I also have `comp!` which makes composition of only two functions, and `ltr` and `rtl` options, just based off what is more intuitive -- `ltr` will execute left to right (so leftmost becomes innermost composed function) and `rtl` is the reverse (leftmost is computed last). 
