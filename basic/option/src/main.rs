mod op;
mod reference;
mod student;
mod null_value;
// 参考： https://www.jianshu.com/p/ce5bddf4b335
fn main() {
    println!("ref 解构 引用的使用：");
    reference::reference();

    println!("Option初始化使用：");
    op::option_basic();

    println!("Option在Struct之中可选字段及函数参数返回值的应用：");
    student::option_struct();

    println!("表示空值的情况：");
    null_value::null_value();

    println!("Option操作：");
    student::operation_op();

    println!("over Option");
}
