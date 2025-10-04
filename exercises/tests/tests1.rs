// tests1.rs
//
// Tests are important to ensure that your code does what you think it should
// do. Tests can be run on this file with the following command: rustlings run
// tests1
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert() {
        assert!(true);
    }
}

fn main() {
    let condition = true;

    if condition {
        println!("Assertion passed!");
    } else {
        println!("Assertion failed!");
        // 模拟 panic，类似 assert! 宏
        panic!("Assertion failed!");
    }

    // 你可以改成 false 来模拟失败
    let condition2 = false;
    if condition2 {
        println!("Second assertion passed!");
    } else {
        println!("Second assertion failed!");
        panic!("Second assertion failed!");
    }
}
