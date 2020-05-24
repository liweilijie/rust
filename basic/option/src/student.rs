#[derive(Debug)]
struct Student {
    name: String,
    year: u8,
    score: Option<f32> // 可选的字段
}

impl Student {
    fn new(n: String, y: u8) -> Self {
        Self {
            name: n,
            year: y,
            score: None,
        }
    }

    // 接收Option作为参数
    fn set_score(&mut self, s: Option<f32>) {
        self.score = s;
    }
}

// 返回Option
fn compute_score(s: f32) -> Option<f32> {
    let d = s * 0.75;
    Some(d)
}

pub fn option_struct() {
    let mut d = Student::new("java".to_string(), 18);
    dbg!(&d.score);

    let score = compute_score(100.0);
    d.set_score(score);
    dbg!(&d.score);
}

pub fn operation_op() {
    let d = Student::new("xiaoming".to_string(), 18);
    let d = Some(d);
    // match 匹配
    match d {
        Some(s) => {
            println!("{:?}", s.name);
        }
        None => {
            println!("None");
        }
    }

    // 我们不关心None,只要其中的值
    let d = Student::new("golang".to_string(), 10);
    let d = Some(&d);
    if let Some(s) = d {
        println!("{:?}", s.name);
    };

    // 注意：断言assert_eq!(a,b)，如果a==b，什么都不发生，否则中断程序。
    // expect: 有值返回值，否则打印注释并且中断程序
    let x = Some("value");
    assert_eq!(x.expect("the world is end."), "value");

    // unwrap()： 有值返回值，否则中断程序
    assert_eq!(x.unwrap(), "value");

    // unwrap_or()： 有值返回值，否则返回一个默认值
   assert_eq!(Some("bike").unwrap_or("bike"), "bike");
   assert_eq!(None.unwrap_or("bike"), "bike");

    // unwrap_or_else(): 有值返回值，否则执行闭包
    let k = 10;
    assert_eq!(Some(4).unwrap_or_else(|| 2 * k), 4);
    assert_eq!(None.unwrap_or_else(|| 2 * k), 20);

    // map(): 改变值，并返回另外一个Option
    let maybe_some_string: Option<String>= Some(String::from("hello, emacs"));
    let maybe_some_len: Option<usize> = maybe_some_string.map(|s| s.len());
    assert_eq!(maybe_some_len, Some(12));

    // map_or(): 有值则执行闭包返回值，否则返回一个自定义默认值
    let x = Some("foo");
    assert_eq!(x.map_or(42, |v|v.len()), 3);

    // map_or_else(): 有值，执行闭包，否则执行另一个闭包
    let k = 21;
    let x = Some("foo");
    assert_eq!(x.map_or_else(||2*k, |v|v.len()), 3);

    let x: Option<&str> = None;
    assert_eq!(x.map_or_else(||2*k, |v|v.len()), 42);

    // ok_or(): 有值，返回Result, 否则返回自定义的错误
    let x = Some("foo");
    assert_eq!(x.ok_or(0), Ok("foo"));

    let x: Option<&str> = None;
    assert_eq!(x.ok_or(0), Err(0));

    // ok_or_else(): 有值，返回Result， 否则执行代表错误的闭包
    let x = Some("foo");
    assert_eq!(x.ok_or_else(||0), Ok("foo"));

    let x: Option<&str> = None;
    assert_eq!(x.ok_or_else(||0), Err(0));

    // iter(): 把Option转换为迭代器
    let x = Some(4);
    assert_eq!(x.iter().next(), Some(&4));

    let x: Option<u32> = None;
    assert_eq!(x.iter().next(), None);

    // and(): 有值，返回另一个Option, 否则返回None
    let x = Some(2);
    let y: Option<&str> = None;
    assert_eq!(x.and(y), None);

    let x: Option<u32> = None;
    let y = Some("foo");
    assert_eq!(x.and(y), None);

    let x: Option<u32> = None;
    let y: Option<&str> = None;
    assert_eq!(x.and(y), None);

    // and_then(): 有值，执行闭包，否则返回None
    fn sq(x: u32) -> Option<u32> { Some(x * x)}
    fn nope(_: u32) -> Option<u32> {None}

    assert_eq!(Some(2).and_then(sq).and_then(sq), Some(16));
    assert_eq!(Some(2).and_then(sq).and_then(nope), None);
    assert_eq!(Some(2).and_then(nope).and_then(sq), None);
    assert_eq!(None.and_then(sq).and_then(sq), None);

    // filter(): 过滤器，过滤出自己想要的值
    fn is_even(n: &i32) -> bool {
        n % 2 == 0
    }
    assert_eq!(None.filter(is_even), None);
    assert_eq!(Some(3).filter(is_even), None);
    assert_eq!(Some(4).filter(is_even), Some(4));

    // or(): 有值，返回自身，否则返回自定义的Option
    let x = Some(2);
    let y = None;
    assert_eq!(x.or(y), Some(2));

    let x = None;
    let y = Some(100);
    assert_eq!(x.or(y), Some(100));

    let x = Some(2);
    let y = Some(100);
    assert_eq!(x.or(y), Some(2));

    let x: Option<u32> = None;
    let y = None;
    assert_eq!(x.or(y), None);

    // or_else(): 有值，返回自身，否则执行闭包
    fn nobody() -> Option<&'static str> { None }
    fn vikings() -> Option<&'static str> { Some("vikings") }
    assert_eq!(Some("barbarians").or_else(vikings), Some("barbarians"));
    assert_eq!(None.or_else(vikings), Some("vikings"));
    assert_eq!(None.or_else(nobody), None);

    // take(): 取出一个值
    let mut x = Some(2);
    let y = x.take();
    assert_eq!(x, None);
    assert_eq!(y, Some(2));

    let mut x: Option<u32> = None;
    let y = x.take();
    assert_eq!(x, None);
    assert_eq!(y, None);
}
