pub mod error;

use crate::error::NameResolutionError;

use ::ty::res::*;
use ast::*;
use span::*;

use anyhow::Result;
use std::collections::HashMap;

pub fn resolve_items<'a>(
    items: &[Item],
    symbol_map: &'a SymbolMap<'a>,
) -> Result<HashMap<Span, Res>> {
    let mut resolver = ASTNameResolver::new(symbol_map);
    resolver.resolve_items(items)?;

    Ok(resolver.finish())
}

pub struct ASTNameResolver<'a> {
    symbol_map: &'a SymbolMap<'a>,
    def_gen: DefIdGen,
    resolution: HashMap<Span, Res>,
    scopes: Vec<HashMap<Symbol, Res>>,
}

impl<'a> ASTNameResolver<'a> {
    pub fn new(symbol_map: &'a SymbolMap<'a>) -> ASTNameResolver {
        ASTNameResolver {
            symbol_map,
            def_gen: DefIdGen::new(),
            resolution: HashMap::new(),
            scopes: Vec::new(),
        }
    }

    pub fn finish(self) -> HashMap<Span, Res> {
        self.resolution
    }

    pub fn insert_define(&mut self, name: Symbol, span: Span, kind: ResKind) -> Result<()> {
        if self.exist_current_scope(&name) {
            let name = self.symbol_map.get(name).to_string();
            return Err(NameResolutionError::DefinedMultipleTimes { name }.into());
        }

        let def = self.def_gen.new_id();
        let res = Res { def, kind };
        self.scopes.last_mut().unwrap().insert(name, res);
        self.resolution.insert(span, res);

        Ok(())
    }

    pub fn insert_use(&mut self, name: Symbol, span: Span) -> Result<()> {
        let res = match self.lookup(&name) {
            Some(res) => res,
            None => {
                let name = self.symbol_map.get(name).to_string();
                return Err(NameResolutionError::UnresolvedNameUsed { name, span }.into());
            }
        };

        self.resolution.insert(span, res);

        Ok(())
    }

    fn lookup(&self, name: &Symbol) -> Option<Res> {
        for scope in self.scopes.iter().rev() {
            if let Some(def) = scope.get(name) {
                return Some(*def);
            }
        }

        None
    }

    fn exist_current_scope(&self, name: &Symbol) -> bool {
        let current_scope = self.scopes.last().unwrap();
        current_scope.contains_key(name)
    }

    pub fn with_new_scope<F>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(&mut ASTNameResolver) -> Result<()>,
    {
        self.scopes.push(HashMap::new());
        f(self)?;
        self.scopes.pop();
        Ok(())
    }
}

impl<'a> ASTNameResolver<'a> {
    pub fn resolve_ident(&mut self, ident: &Ident) -> Result<()> {
        self.insert_use(ident.name, ident.span)?;

        Ok(())
    }

    pub fn resolve_expr(&mut self, expr: &Expr) -> Result<()> {
        match expr {
            Expr::Call { fun, args } => {
                self.resolve_expr(fun)?;

                for arg in args {
                    self.resolve_expr(arg)?;
                }
            }
            Expr::Binary { lhs, rhs, .. } => {
                self.resolve_expr(lhs)?;
                self.resolve_expr(rhs)?;
            }
            Expr::Unary { expr, .. } => self.resolve_expr(expr)?,
            Expr::If {
                cond,
                then,
                else_opt,
            } => {
                self.resolve_expr(cond)?;
                self.resolve_block(then)?;

                if let Some(else_expr) = else_opt {
                    self.resolve_expr(else_expr)?;
                }
            }
            Expr::Loop { block } => self.resolve_block(block)?,
            Expr::Break { expr } | Expr::Continue { expr } | Expr::Return { expr } => {
                if let Some(expr) = expr {
                    self.resolve_expr(expr)?
                }
            }
            Expr::Block { block } => self.resolve_block(block)?,
            Expr::Assign { lhs, rhs } => {
                self.resolve_expr(rhs)?;
                self.resolve_expr(lhs)?;
            }
            Expr::Lit { .. } => {}
            Expr::Path(path) => self.resolve_ident(&path.ident)?,
        }

        Ok(())
    }

    pub fn resolve_block(&mut self, block: &Block) -> Result<()> {
        self.with_new_scope(|this| {
            for stmt in &block.stmts {
                this.resolve_stmt(stmt)?;
            }
            Ok(())
        })
    }

    pub fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::Local { ident, init, .. } => {
                self.resolve_expr(init)?;
                self.insert_define(ident.name, ident.span, ResKind::Local)?;
            }
            Stmt::Expr(expr) | Stmt::Semi(expr) | Stmt::Println(expr) => self.resolve_expr(expr)?,
        }

        Ok(())
    }

    pub fn resolve_items(&mut self, items: &[Item]) -> Result<()> {
        self.with_new_scope(|this| {
            // resolve item declaration.
            for item in items {
                let ident = &item.ident;

                let kind = match item.kind {
                    ItemKind::Fn(_) => ResKind::Fn,
                };

                this.insert_define(ident.name, ident.span, kind)?;
            }

            this.with_new_scope(|this| {
                for item in items {
                    match &item.kind {
                        ItemKind::Fn(fun) => this.resolve_item_fn(fun.as_ref())?,
                    }
                }
                Ok(())
            })
        })
    }

    pub fn resolve_item_fn(&mut self, fun: &Fn) -> Result<()> {
        self.with_new_scope(|this| {
            for param in &fun.inputs {
                let ident = &param.ident;
                this.insert_define(ident.name, ident.span, ResKind::Local)?;
            }

            this.resolve_block(&fun.body)
        })
    }
}
