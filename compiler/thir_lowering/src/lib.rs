#[allow(unused_imports)]
use ir::{constant::*, stmt::*, *};

use std::collections::HashMap;

#[allow(dead_code)]
pub struct LoweringContext {
    body: Body,

    block_at: BlockId,

    local_name_table: HashMap<String, Place>,
}

impl LoweringContext {
    pub fn new() -> Self {
        LoweringContext {
            body: Body::new(),
            block_at: BlockId::dummy(),
            local_name_table: HashMap::new(),
        }
    }

    pub fn build(self) -> Body {
        self.body
    }
}
