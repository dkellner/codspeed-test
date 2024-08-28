pub fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }

    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let tmp = b;
        b += a;
        a = tmp;
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(fibonacci(6), 13);
        assert_eq!(fibonacci(7), 21);
    }
}
