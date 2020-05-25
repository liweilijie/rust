mod ownership;
mod slice;
fn main() {
    ownership::what_is_ownership();
    ownership::references_and_borrowing();
    slice::slice_example();
    println!("Hello, world!");
}
