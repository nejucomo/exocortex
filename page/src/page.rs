#[derive(Debug)]
pub enum Page<'a> {
    ReadOnly(&'static str),
    ReadWrite(&'a mut String),
}
