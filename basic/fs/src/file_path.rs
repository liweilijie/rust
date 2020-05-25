use std::path::Path;
use std::io;

pub fn path_common() {
    // 从 `&'static str`创建一个`Path`
    let path = Path::new(".");

    // `display`方法返回一个可显示(showable)的结构体
    let display = path.display();
    println!("display:{}", display);

    // `join`
    let new_path = path.join("a").join("b");

    // 将路径转换成一个字符串切片
    match new_path.to_str() {
        None => println!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}

pub fn get_executable_path() -> Result<String, io::Error> {
    println!("{:?}", std::env::current_exe());
    match std::env::current_exe() {
        Ok(exe_path) => {
            println!("Path of this executable is: {}", exe_path.display());
            match exe_path.to_str() {
                Some(s) => Ok(String::from(s)),
                None => Ok(String::new()),
            }
        },
        Err(e) => {
            println!("failed to get current exe path: {}", e);
            Err(e)
        }
    }
}
