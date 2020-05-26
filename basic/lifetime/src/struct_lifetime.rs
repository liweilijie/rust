#[derive(Debug)]
struct Foo<'a> {
    v: &'a i32
}

pub fn struct_lifetime_foo() {
    let v = 123;
    let foo = Foo {
        v: &v
    };
    println!("foo: {:?}", foo);
}
