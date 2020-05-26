pub fn static_lifetime_default() {
    // 所有的字符串字面值都是 'static 生命周期，例如：
    let s: &'static str = "codercat is a static lifetime.";
    // 上面代码中的生命周期参数可以省略，就变成如下形式：
    let s: &str = "codercat is a static lifetime.";
    // 还有static变量的生命周期也是'static。
    static V: i32 = 123;
    println!("s:{}, v:{}", s, V);

    static N: i32 = 789;
    let max = max_num(&V, &N);
    println!("static max:{}", max);
}
// 可以对函数的参数进行限制，让其只能只能接受静态变量：
// 在使用static生命周期参数的时候，由于它已经内置在编译器中了，所以不需要进行声明。
// 在结构体中的使用方式也类似就不再次举例了。
fn max_num(x: &'static i32, y: &'static i32) -> &'static i32 {
    if x > y {
        &x
    } else {
        &y
    }
}