use mir::{stmt::*, terminator::*, *};

#[allow(dead_code)]
pub(crate) struct MirBuilder {
    body: Body,
    tail: BlockId,
}

#[allow(dead_code)]
impl MirBuilder {
    pub(crate) fn new() -> Self {
        Self {
            body: Body::new(),
            tail: BlockId::dummy(),
        }
    }

    pub(crate) fn tail(&self) -> BlockId {
        self.tail
    }

    pub(crate) fn push_local_decl(&mut self, decl: LocalDecl) -> Place {
        let id = self.body.local_decls.push_and_get_key(decl);
        Place::new(id)
    }

    pub(crate) fn build(self) -> Body {
        self.body
    }

    pub(crate) fn push_block(&mut self, terminator: Option<Terminator>) -> BlockId {
        self.tail = self.body.blocks.push_and_get_key(Block::new(terminator));
        self.tail
    }

    pub(crate) fn set_terminator(&mut self, target: BlockId, terminator: Terminator) {
        let block = self
            .body
            .blocks
            .get_mut(target)
            .expect("Given target is invalid.");
        block.terminator = Some(terminator);
    }

    pub(crate) fn push_stmt(&mut self, target: BlockId, stmt: Statement) {
        self.body.blocks[target].stmts.push(stmt);
    }
}
