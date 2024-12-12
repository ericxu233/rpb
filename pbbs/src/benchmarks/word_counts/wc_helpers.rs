use crate::DefChar;

#[derive(Clone, Hash, Copy, Debug, Eq, PartialEq)]
pub struct DefWord<'a>(&'a [DefChar]);

impl<'a> Default for DefWord<'a> {
    fn default() -> Self {
        DefWord(&[])
    }
}

impl <'a> std::fmt::Display for DefWord<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for c in self.0 {
            write!(f, "{}", *c as char)?;
        }
        Ok(())
    }
}
impl <'a> DefWord<'a> {
    pub fn new(s: &'a [DefChar]) -> Self {
        DefWord(s)
    }
}