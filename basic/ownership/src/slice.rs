pub fn slice_example() {
    let my_string = String::from("hello world");

    // first_word 中传入`String`的slice
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world.";

    // first_word中传入字符串字面值的slice
    let word = first_word(&my_string_literal[..]);

    // 因为字符串字面值 **就是** 字符串 slice，
    // 这样写也可以，即不使用 slice 语法！
    let word = first_word(my_string_literal);
    println!("word:{}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
