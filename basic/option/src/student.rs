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
