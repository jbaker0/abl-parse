#[derive(Clone, Debug, PartialEq)]
pub enum Statement<'a> {
    Procedure(Box<Vec<Statement<'a>>>),
    Function(Box<Vec<Statement<'a>>>),
    Include(Include<'a>),
    Comment(&'a str),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Include<'a> {
    pub path: &'a str,
    pub content: Option<&'a str>,
    pub arguments: Vec<&'a str>,
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
