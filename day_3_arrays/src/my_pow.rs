impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n>= 0 {
            return exponent(x, n);
        }
        return (1 as f32/exponent(-x, n) as f32) as f64
    }
}

pub fn exponent(x:f64, n:i32) -> f64{
    if n == 0 {
        return 1 as f64;
    }
    if n == 1 {
        return x;
    }
    // When n is odd
    let val = exponent(x, n/2);
    if n&1 > 0 {
        return x * val * val;
    }
    return val*val;
}