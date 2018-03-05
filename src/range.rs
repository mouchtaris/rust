#[derive(Debug, Clone, Copy)]
pub struct Range {
    pub from    : i32,
    pub to      : i32,
    pub step    : i32,
}

impl Range {
    pub fn next(&self) -> Range { Range { from: self.from + self.step, .. *self } }
}
