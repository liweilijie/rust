mod readfile;
// Option和Result区别：
// Option是可能存在空值
// Result是可能存在错误
fn main() {
    println!("Hello, Result!");
    if let Ok(content) = readfile::read_username_from_file() {
        println!("{}", content);
    }
    // readfile::read_username_from_file_commend();
    // readfile::read_username_from_file_commend2();
    // readfile::read_username_from_file_over();
}
