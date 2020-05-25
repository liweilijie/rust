use crate::fs::list_dir;

mod file_path;
mod fs;

fn main() {
    file_path::get_executable_path();
    file_path::path_common();
    fs::open_and_read();
    match fs::readline_and_print() {
        Ok(()) => println!("it's ok"),
        Err(err) => println!("err:{}", err)
    }
    match fs::file_append() {
        Ok(()) => println!("append ok."),
        Err(e) => println!("append error:{}", e)
    }
    list_dir();
    println!("Hello, world!");
}
