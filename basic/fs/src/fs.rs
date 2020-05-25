use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

pub fn open_and_read() {
    let path = Path::new("hello.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => {
            println!("couldn't open {}: {}", display, why.description());
            return;
        },
        Ok(f) => f,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(e) => {
            println!("couldn't read {}:{}", display, e.description());
            return;
        },
        Ok(_) => {
            println!("{} contains:\n{}", display, s);
        }
    }
}


use std::io::{self, BufReader};
// 从文件按行读取内容，打印输出
pub fn readline_and_print() -> io::Result<()> {
    let f = File::open("/Users/liwei/coding/rust/git/rust/basic/fs/Cargo.toml")?;
    let f = BufReader::new(f);

    for line in f.lines() {
        if let Ok(line) = line {
            println!("{:?}", line);
        }
    }
    Ok(())
}

// 将内容写入文件
// 打开文件可以指定多个参数，以下例子可读可写，
// create(true)是如果文件不存在则创建文件，存在则使用这个文件，
// create_new(true)的作用是，当文件存在时，会报错，Error { repr: Os { code: 17, message: "File exists" } }，
// 不存在则新建文件，
// 并且指定append追加写入，
// 打开文件，将文件句柄赋值给file.
pub fn file_append() -> io::Result<()> {
    let filename = "foo.txt";
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        // .create_new(true)
        .append(true)
        // .truncate(true)
        .open(filename);

    match file {
        Ok(mut stream) => {
            stream.write_all(b"hello, world!\n")?;
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
    Ok(())
}

// 获取目录列表
// 对文件进行操作，很可能会读取目录列表，使用fs::read_dir方法，可以获取目录列表及文件相关属性
use std::fs;
pub fn list_dir() {
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries {
            println!("entry:{:?}", entry);
            if let Ok(entry) = entry {
                println!("path:{:?}", entry.path());
                println!("file_name:{:?}", entry.file_name());
                println!("file_type:{:?}", entry.file_type());
            }
        }
    }
}