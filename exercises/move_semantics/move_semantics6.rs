// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
// 解法：使用引用&，允许使用值但不获取其所有权
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
// 解法：取消引用，占有data的所有权
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
