pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            // * SWAP
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

fn main() {
    println!("Hello world!");
}

#[cfg(test)]
mod tests {

    use super::{gcd, main};

    #[test]
    fn test_main() {
        // main();
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(14, 15), 1);

        assert_eq!(gcd(2*3*5*11*17, 3*7*11*13*19), 3*11);
    }
}
