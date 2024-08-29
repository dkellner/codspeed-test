/// Compute the nth element in the Fibonacci sequence.
pub fn fibonacci(n: u16) -> u64 {
    let n: i32 = n.into();
    let sq5 = 5.0f64.sqrt();
    let result = 1.0 / sq5 * ((1.0 + sq5) / 2.0).powi(n) - 1.0 / sq5 * ((1.0 - sq5) / 2.0).powi(n);
    result.round() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(6), 8);
        assert_eq!(fibonacci(7), 13);
    }
}
