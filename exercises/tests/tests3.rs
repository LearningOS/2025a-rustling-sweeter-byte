// tests3.rs
//
// This test isn't testing our function -- make it do that in such a way that
// the test passes. Then write a second test that tests whether we get the
// result we expect to get when we call `is_even(5)`.
//
// Execute `rustlings hint tests3` or use the `hint` watch subcommand for a
// hint.


pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(4)); // 偶数 → true
        assert!(is_even(0)); // 0 也是偶数
        assert!(is_even(-2)); // 负偶数
    }

    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(5));  // 奇数 → false
        assert!(!is_even(1));  // 奇数
        assert!(!is_even(-3)); // 负奇数
    }
}

fn main() {
    let numbers = [0, 1, 2, 3, 4, 5, -2, -3];

    for &num in &numbers {
        println!("{} is even? {}", num, is_even(num));
    }
}
