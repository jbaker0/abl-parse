pub enum Statement {
    Procedure(Box<Vec<Statement>>),
    Function(Box<Vec<Statement>>),
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
