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
}
