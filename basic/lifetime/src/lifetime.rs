pub fn lifetime_max() {
    let x = 1;
    let y = 8;
    let max = max_num(&x, &y);
    println!("max: {}", max);

    let x = 99;
    let y = 10;
    let max = max_num_include(&x, &y);
    println!("max: {}", max);
}

fn max_num<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y {
        &x
    } else {
        &y
    }
}
// 如果函数参数的生命周期参数与函数返回值的生命周期参数不建关联的话，那么生命周期参数就没有任何意义。
// 例如：
/*
fn max_num2<'a, 'b>(x: &'a i32, y: &'a i32) -> &'b i32 {
    if x > y {
        &x
    } else {
        &y
    }
}
 */
// 上面代码中函数返回值的生命周期参数'b'是未知的，编译器不知道这个'b'的具体生命周期是多少，所以没有任何意义。
// 可以显示指明多个生命周期参数间的关系，从而使每个生命周期参数都有一个具体的的生命周期。例如：
// 下面代码使用'b: 'a来标注'a与'b之间的生命周期关系，它表示'a的生命周期不能超过'b，
// 即函数返回值的生命周期'a（借用方）不能超过'b``（出借方），'a也不会超过'a`（出借方）。
fn max_num_include<'a, 'b: 'a>(x: &'a i32, y: &'b i32) -> &'a i32 {
    if x > y {
        &x
    } else {
        &y
    }
}

