// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let data = "Rust is great!".to_string();

    // 传入借用（不取得所有权）
    get_char(&data);

    // 传入所有权（会被函数接收）
    string_uppercase(data);
}

// Should not take ownership — 改为借用
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership — 保持按值接收
fn string_uppercase(data: String) {
    // to_uppercase() 返回新的 String，我们直接打印这个返回值
    println!("{}", data.to_uppercase());
}

