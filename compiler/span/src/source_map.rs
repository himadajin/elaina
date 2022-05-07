use crate::*;

pub struct SourceMap {
    file: SourceFile,
}

impl SourceMap {
    pub fn new(file: SourceFile) -> Self {
        Self { file }
    }

    pub fn span_to_string(&self, sp: Span) -> String {
        let data = sp.data();
        format!("{}:{}:{}", self.file.name, data.lo, data.hi)
    }
}

#[derive(Clone)]
pub struct SourceFile {
    pub name: String,
    pub src: String,
}
