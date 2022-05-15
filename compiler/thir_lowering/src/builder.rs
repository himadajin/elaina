use mir::{stmt::*, terminator::*, *};
use span::Symbol;
use ty::res::DefId;

#[allow(dead_code)]
pub(crate) struct MirBuilder<'tcx> {
    body: Body<'tcx>,
}

#[allow(dead_code)]
impl<'tcx> MirBuilder<'tcx> {
    pub(crate) fn new(def: DefId, name: Symbol) -> Self {
        Self {
            body: Body::new(def, name),
        }
    }

    pub(crate) fn set_arg_count(&mut self, count: usize) {
        self.body.arg_count = count;
    }

    pub(crate) fn push_local_decl(&mut self, decl: LocalDecl<'tcx>) -> Place {
        let id = self.body.local_decls.push_and_get_key(decl);
        Place::new(id)
    }

    pub(crate) fn build(self) -> Body<'tcx> {
        self.body
    }

    pub(crate) fn push_block(&mut self, terminator: Option<Terminator<'tcx>>) -> BlockId {
        self.body.blocks.push_and_get_key(Block::new(terminator))
    }

    pub(crate) fn set_terminator(&mut self, target: BlockId, terminator: Terminator<'tcx>) {
        let block = self
            .body
            .blocks
            .get_mut(target)
            .expect("Given target is invalid.");
        block.terminator = Some(terminator);
    }

    pub(crate) fn push_stmt(&mut self, target: BlockId, stmt: Statement<'tcx>) {
        self.body.blocks[target].stmts.push(stmt);
    }
}
