/*
// Option的类型定义：
pub enum Option<T> {
    None,
    Some(T),
}
 */
pub fn option_basic() {
    println!("option basic to use.");

    // pub fn is_some(&self) -> bool
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_some(), true);
    let x: Option<u32> = None;
    assert_eq!(x.is_some(), false);

    // pub fn is_none(&self) -> bool
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_none(), false);

    // 初始化值
    let s = Some(3);
    if let Some(ref s) = s {
        println!("{:?}", *s);
    }


}
