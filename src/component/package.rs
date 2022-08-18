#[derive(Clone)]
pub struct PkgInfo {
    pub name: Box<str>,
    pub assignee: Box<str>,
    pub marks: Vec<String>, // TODO: replace String with informative struc type
}
