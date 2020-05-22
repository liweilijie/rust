pub fn null_value() {
    let op = None;
    check_optional(op);

    let op = Some(Box::new(90000));
    check_optional(op);

}

fn check_optional(op: Option<Box<i32>>) {
    match op {
        Some(ref p) => println!("has value {}", p),
        None => println!("has no value"),
    }
}
