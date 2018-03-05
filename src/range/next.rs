use super::Range;

impl Range {
    pub fn next(&self) -> Range { Range { from: self.from + self.step, .. *self } }
}
