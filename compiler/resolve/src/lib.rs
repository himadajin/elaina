use std::collections::HashMap;

use ast::*;
use hir::res::*;
use span::*;

pub fn resolve_items(items: &[Item]) -> HashMap<Span, Res> {
    let mut resolver = ASTNameResolver::new();
    resolver.resolve_items(items);

    resolver.finish()
}

pub struct ASTNameResolver {
    def_gen: DefIdGen,
    resolution: HashMap<Span, Res>,
    scopes: Vec<HashMap<Symbol, Res>>,
}

impl ASTNameResolver {
    pub fn new() -> ASTNameResolver {
        ASTNameResolver {
            def_gen: DefIdGen::new(),
            resolution: HashMap::new(),
            scopes: Vec::new(),
        }
    }

    pub fn finish(self) -> HashMap<Span, Res> {
        self.resolution
    }

    pub fn new_decl(&mut self, name: Symbol, span: Span, kind: ResKind) {
        let def = self.def_gen.new_id();
        let res = Res { def, kind };
        self.scopes.last_mut().unwrap().insert(name, res);
        self.resolution.insert(span, res);
    }

    pub fn new_use(&mut self, name: Symbol, span: Span) {
        let res = self.lookup(&name).expect("Undefined ident given.");
        self.resolution.insert(span, res);
    }

    fn lookup(&self, name: &Symbol) -> Option<Res> {
        for scope in self.scopes.iter().rev() {
            if let Some(def) = scope.get(name) {
                return Some(*def);
            }
        }

        None
    }

    pub fn with_new_scope<F>(&mut self, f: F)
    where
        F: FnOnce(&mut ASTNameResolver),
    {
        self.scopes.push(HashMap::new());
        f(self);
        self.scopes.pop();
    }
}

impl ASTNameResolver {
    pub fn resolve_ident(&mut self, ident: &Ident) {
        self.new_use(ident.name, ident.span);
    }

    pub fn resolve_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Call { fun, args } => {
                self.resolve_expr(fun);

                for arg in args {
                    self.resolve_expr(arg);
                }
            }
            Expr::Binary { lhs, rhs, .. } => {
                self.resolve_expr(lhs);
                self.resolve_expr(rhs);
            }
            Expr::Unary { expr, .. } => self.resolve_expr(expr),
            Expr::If {
                cond,
                then,
                else_opt,
            } => {
                self.resolve_expr(cond);
                self.resolve_block(then);

                if let Some(else_expr) = else_opt {
                    self.resolve_expr(else_expr);
                }
            }
            Expr::Loop { block } => self.resolve_block(block),
            Expr::Break { expr } | Expr::Continue { expr } => {
                if let Some(expr) = expr {
                    self.resolve_expr(expr)
                }
            }
            Expr::Block { block } => self.resolve_block(block),
            Expr::Assign { lhs, rhs } => {
                self.resolve_expr(rhs);
                self.resolve_expr(lhs);
            }
            Expr::Lit { .. } => {}
            Expr::Path(path) => self.resolve_ident(&path.ident),
        }
    }

    pub fn resolve_block(&mut self, block: &Block) {
        self.with_new_scope(|this| {
            for stmt in &block.stmts {
                this.resolve_stmt(stmt);
            }
        });
    }

    pub fn resolve_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Local { ident, init, .. } => {
                self.resolve_expr(init);
                self.new_decl(ident.name, ident.span, ResKind::Local);
            }
            Stmt::Expr(expr) | Stmt::Semi(expr) | Stmt::Println(expr) => self.resolve_expr(expr),
        }
    }

    pub fn resolve_items(&mut self, items: &[Item]) {
        self.with_new_scope(|this| {
            // resolve item declaration.
            for item in items {
                let ident = &item.ident;

                let kind = match item.kind {
                    ItemKind::Fn(_) => ResKind::Fn,
                };

                this.new_decl(ident.name, ident.span, kind);
            }

            this.with_new_scope(|this| {
                for item in items {
                    match &item.kind {
                        ItemKind::Fn(fun) => this.resolve_item_fn(fun.as_ref()),
                    }
                }
            });
        })
    }

    pub fn resolve_item_fn(&mut self, fun: &Fn) {
        self.with_new_scope(|this| {
            for param in &fun.inputs {
                let ident = &param.ident;
                this.new_decl(ident.name, ident.span, ResKind::Local);
            }

            this.resolve_block(&fun.body);
        })
    }
}
