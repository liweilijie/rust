mod lifetime;
mod struct_lifetime;
mod static_lifetime;
fn main() {
    lifetime::lifetime_max();
    struct_lifetime::struct_lifetime_foo();
    println!("Hello, world!");
}
