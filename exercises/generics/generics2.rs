// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.


struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}

fn main() {
    // 包装一个 u32
    let int_wrapper = Wrapper::new(42u32);
    println!("int_wrapper 包装的值: {}", int_wrapper.value);

    // 包装一个字符串
    let str_wrapper = Wrapper::new("Hello, Rust!");
    println!("str_wrapper 包装的值: {}", str_wrapper.value);

    // 包装一个浮点数
    let float_wrapper = Wrapper::new(3.14f64);
    println!("float_wrapper 包装的值: {}", float_wrapper.value);
}
