#[derive(Clone, Debug, PartialEq)]
pub enum Statement<'a> {
    Procedure(Box<Vec<Statement<'a>>>),
    Function(Box<Vec<Statement<'a>>>),
    Include(Include<'a>),
    Comment(&'a str),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Include<'a> {
    pub(crate) path: &'a str,
    pub(crate) content: Option<&'a str>,
    pub(crate) arguments: Vec<&'a str>,
}

impl Default for Include<'_> {
    fn default() -> Self {
        Self {
            path: "",
            content: None,
            arguments: Vec::new(),
        }
    }
}
