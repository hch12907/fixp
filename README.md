## fixp
`fixp` is a Rust library that provides a simple implementation of fixed point arithmetic. Currently, it is not being published on crates.io, simply because the crate in its current state, is far from being usable in production.

This is also a pet project of mine, so frequent updates should not be expected. (sorry for that)

### Support formats:
Right now, only 2 formats are supported:
* Q16.16
* Q22.10

### Usage:
Since this library is not published as crate, you have to find a way to include it yourself (again, sorry for that).
```
use fixp::Q22_10;

fn main() {
    // From<i32> and From<f32> is implemented for both formats.
    let a = Q22_10::from(100.0f32);
    let b = Q22_10::from(20i32);

    let add = a + b;
    let sub = a - b;
    let mul = a * b;
    let div = a / b;

    assert_eq!(add, Q22_10::from( 120.0f32));
    assert_eq!(sub, Q22_10::from(  80.0f32));
    assert_eq!(mul, Q22_10::from(2000.0f32));
    assert_eq!(div, Q22_10::from(   5.0f32));
}

```

<br>
<br>

## fixp （中文版 README）
此库旨在为Rust语言提供一套定点数实现。目前，本库仅支持两种定点数格式，即Q22.10以及Q16.16。此库功能尚不完整，不适合在生产环境中使用，故暂不将其发布在crates.io上。此外，此库属于本人的小项目之一，更新频率将因此不高，有请见谅。虽说如此，若欲尝试此库，你可以把此库当作本地crate使用；具体使用方法见上。
