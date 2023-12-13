use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct Commit {
    pub(crate) message: String,
    pub(crate) author: String,
}

pub(crate) struct ParsedScope {
    pub(crate) scope: String,
    pub(crate) type_: String,
    pub(crate) author: Option<String>,
}

pub(crate) type PreparedScope = HashMap<
    String,
    (
        u32,
        HashMap<String, (u32, std::collections::HashMap<String, u32>)>,
    ),
>;
pub(crate) type Scope = Vec<(
    String,
    (
        u32,
        HashMap<String, (u32, std::collections::HashMap<String, u32>)>,
    ),
)>;

#[derive(Debug)]
pub(crate) struct Analysis {
    pub(crate) types: HashMap<String, u32>,
    pub(crate) authors: HashMap<String, u32>,
    pub(crate) scope: Scope,
}
