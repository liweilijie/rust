use std::io;
use std::io::Read;
use std::fs::File;
use std::fs;

// 当编写一个函数，但是该函数可能会失败，此时除了在函数中处理错误外，还可以将错误传给调用者，
// 让调用者决定如何处理，这被称为传播错误。
pub fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 传播错误的简写方式，提倡的方式
pub fn read_username_from_file_commend() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 更进一步简写
pub fn read_username_from_file_commend2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?
        .read_to_string(&mut s)?;
    Ok(s)
}

// rust提供了fs::read_to_string函数
pub fn read_username_from_file_over() -> io::Result<String> {
    // pub fn read_username_from_file_over() -> Result(String, io::Error) {
    fs::read_to_string("hello.txt")
}
