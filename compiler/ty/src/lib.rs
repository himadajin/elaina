pub mod constant;
pub mod res;

pub use constant::*;

use res::DefId;
use span::{Symbol, SymbolMap};

use std::collections::HashMap;
use std::ops::Deref;
use typed_arena::Arena;

pub struct TyCtx<'ast, 'tcx> {
    arena: &'tcx TyArena<'tcx>,
    interner: Interner<'tcx>,
    pub common_types: CommonTypes<'tcx>,
    pub common_consts: CommonConsts<'tcx>,
    pub symbol_map: &'ast SymbolMap<'ast>,
    pub def_map: HashMap<DefId, Ty<'tcx>>,
}

impl<'ast, 'tcx> TyCtx<'ast, 'tcx> {
    pub fn new(arena: &'tcx TyArena<'tcx>, symbol_map: &'ast SymbolMap<'ast>) -> TyCtx<'ast, 'tcx> {
        let interner = Interner::new(arena);
        let common_types = CommonTypes::new(&interner);
        let common_consts = CommonConsts::new(&interner, &common_types);

        TyCtx {
            arena,
            interner,
            common_types,
            common_consts,
            symbol_map,
            def_map: HashMap::new(),
        }
    }

    pub fn intern(&self, kind: TyKind<'tcx>) -> Ty<'tcx> {
        self.interner.intern_ty(kind)
    }

    pub fn intern_tuple(&self, fields: Vec<Ty<'tcx>>) -> Ty<'tcx> {
        let fields = self.arena.tuple_fields.alloc(fields);
        self.intern(TyKind::Tuple(fields.as_slice()))
    }

    pub fn common_type_from_name(&self, name: Symbol) -> Ty<'tcx> {
        match self.symbol_map.get(name) {
            "bool" => self.common_types.bool,
            "i32" => self.common_types.i32,
            unknown => panic!("The type {} does not exist", unknown),
        }
    }

    pub fn intern_const(&self, value: ConstValue<'tcx>) -> Const<'tcx> {
        self.interner.intern_const(value)
    }

    pub fn intern_const_zst(&self, ty: Ty<'tcx>) -> Const<'tcx> {
        self.interner.intern_const(ConstValue {
            ty,
            literal: ConstLit::ZST,
        })
    }
}

// pub fn with_context<'ast, F, T>(symbol_map: &'ast SymbolMap<'ast>, f: F) -> T
// where
//     F: FnOnce(&mut TyCtx) -> T,
// {
//     let arena = TyArena::new();
//     let mut context = TyCtx::new(&arena, symbol_map);
//     f(&mut context)
// }

pub struct TyArena<'tcx> {
    types: Arena<TyKind<'tcx>>,
    tuple_fields: Arena<Vec<Ty<'tcx>>>,
    consts: Arena<ConstValue<'tcx>>,
}

impl<'tcx> TyArena<'tcx> {
    pub fn new() -> TyArena<'tcx> {
        TyArena {
            types: Arena::new(),
            tuple_fields: Arena::new(),
            consts: Arena::new(),
        }
    }
}

struct Interner<'tcx> {
    arena: &'tcx TyArena<'tcx>,
}

impl<'tcx> Interner<'tcx> {
    fn new(arena: &'tcx TyArena<'tcx>) -> Interner<'tcx> {
        Interner { arena }
    }

    fn intern_ty(&self, kind: TyKind<'tcx>) -> Ty<'tcx> {
        Ty(self.arena.types.alloc(kind))
    }

    fn intern_const(&self, value: ConstValue<'tcx>) -> Const<'tcx> {
        Const(self.arena.consts.alloc(value))
    }
}

pub struct CommonTypes<'tcx> {
    pub unit: Ty<'tcx>,
    pub bool: Ty<'tcx>,
    pub i32: Ty<'tcx>,
    pub never: Ty<'tcx>,
}

impl<'tcx> CommonTypes<'tcx> {
    fn new(interner: &Interner<'tcx>) -> CommonTypes<'tcx> {
        let mk = |kind| interner.intern_ty(kind);

        CommonTypes {
            unit: mk(TyKind::Tuple(&[])),
            bool: mk(TyKind::Bool),
            i32: mk(TyKind::Int(IntTy::I32)),
            never: mk(TyKind::Never),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ty<'tcx>(&'tcx TyKind<'tcx>);

impl<'tcx> Ty<'tcx> {
    #[inline]
    pub fn kind(&self) -> &'tcx TyKind<'tcx> {
        self.0
    }
}

impl<'tcx> Deref for Ty<'tcx> {
    type Target = TyKind<'tcx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.kind()
    }
}

impl<'tcx> Ty<'tcx> {
    pub fn is_zst(&self) -> bool {
        match &self.0 {
            TyKind::Tuple(ts) => ts.is_empty(),
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TyKind<'tcx> {
    Bool,

    Int(IntTy),

    Tuple(&'tcx [Ty<'tcx>]),

    FnDef(DefId),

    Never,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntTy {
    I32,
}
