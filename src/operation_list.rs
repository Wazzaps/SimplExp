use crate::expressions::{ExprOpRef, ExprPart, ExprPartRef};
use serde::{Serialize, Serializer};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct OperationList {
    pub ops: Vec<ExprOpRef>,
    pub ops_set: HashMap<ExprOpRef, usize>,
}

impl OperationList {
    pub fn new() -> Self {
        OperationList {
            ops: Vec::new(),
            ops_set: HashMap::new(),
        }
    }

    pub fn from(op: &ExprPart) -> Self {
        let mut ops = OperationList::new();
        ops.add(op);
        ops
    }

    pub fn add(&mut self, op: &ExprPart) -> ExprPartRef {
        match op {
            ExprPart::IntLiteral(v) => ExprPartRef::IntLiteral(*v),
            ExprPart::FloatLiteral(v) => ExprPartRef::FloatLiteral((*v).into()),
            ExprPart::StringLiteral(v) => ExprPartRef::StringLiteral(v.clone()),
            ExprPart::Operation(op) => {
                let op_ref = op.to_expr_op_ref(self);

                let ops_set = &mut self.ops_set;
                let ops = &mut self.ops;

                let idx = ops_set.entry(op_ref.clone()).or_insert_with(move || {
                    let len = ops.len();
                    ops.push(op_ref);
                    len
                });
                ExprPartRef::Operation(*idx)
            }
        }
    }
}

impl Default for OperationList {
    fn default() -> Self {
        OperationList::new()
    }
}

impl Serialize for OperationList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.ops.serialize(serializer)
    }
}
