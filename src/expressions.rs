use crate::c_api::CloneFromPtr;
use crate::operation_list::OperationList;
use eq_float::F32;
use num_derive::FromPrimitive;
use serde::Serialize;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

#[derive(FromPrimitive)]
pub enum ExprOpId {
    Add = 1,
    Mul = 2,
    Div = 3,
    Fdiv = 4,
    Mod = 5,
    Pow = 6,
    Eq = 7,
    Neq = 8,
    Lt = 9,
    Lte = 10,
    Gt = 11,
    Gte = 12,
    BAnd = 13,
    BOr = 14,
    Neg = 15,
    BInvert = 16,
    Min = 17,
    Max = 18,
    Abs = 19,
    ToStr = 20,
    MeasureTextX = 21,
    MeasureTextY = 22,
}

#[derive(Serialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ExprOp {
    Var {
        name: String,
    },
    Add {
        a: Arc<ExprPart>,
        b: Arc<ExprPart>,
    },
    Mul {
        a: Arc<ExprPart>,
        b: Arc<ExprPart>,
    },
    Div {
        a: Arc<ExprPart>,
        b: Arc<ExprPart>,
    },
    Fdiv {
        a: Arc<ExprPart>,
        b: Arc<ExprPart>,
    },
    Mod {
        a: Arc<ExprPart>,
        b: Arc<ExprPart>,
    },
    Pow {
        a: Arc<ExprPart>,
        b: Arc<ExprPart>,
    },
    Eq {
        a: Arc<ExprPart>,
        b: Arc<ExprPart>,
    },
    Neq {
        a: Arc<ExprPart>,
        b: Arc<ExprPart>,
    },
    Lt {
        a: Arc<ExprPart>,
        b: Arc<ExprPart>,
    },
    Lte {
        a: Arc<ExprPart>,
        b: Arc<ExprPart>,
    },
    Gt {
        a: Arc<ExprPart>,
        b: Arc<ExprPart>,
    },
    Gte {
        a: Arc<ExprPart>,
        b: Arc<ExprPart>,
    },
    BAnd {
        a: Arc<ExprPart>,
        b: Arc<ExprPart>,
    },
    BOr {
        a: Arc<ExprPart>,
        b: Arc<ExprPart>,
    },
    Neg {
        a: Arc<ExprPart>,
    },
    BInvert {
        a: Arc<ExprPart>,
    },
    Min {
        a: Arc<ExprPart>,
        b: Arc<ExprPart>,
    },
    Max {
        a: Arc<ExprPart>,
        b: Arc<ExprPart>,
    },
    Abs {
        a: Arc<ExprPart>,
    },
    Inf,
    ToStr {
        a: Arc<ExprPart>,
    },
    #[serde(rename_all = "camelCase")]
    MeasureTextX {
        text: Arc<ExprPart>,
        font_size: Arc<ExprPart>,
    },
    #[serde(rename_all = "camelCase")]
    MeasureTextY {
        text: Arc<ExprPart>,
        font_size: Arc<ExprPart>,
    },
}

#[derive(Serialize, Debug, Clone, Hash, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ExprOpRef {
    Var {
        name: String,
    },
    Add {
        a: usize,
        b: usize,
    },
    Mul {
        a: usize,
        b: usize,
    },
    Div {
        a: usize,
        b: usize,
    },
    Fdiv {
        a: usize,
        b: usize,
    },
    Mod {
        a: usize,
        b: usize,
    },
    Pow {
        a: usize,
        b: usize,
    },
    Eq {
        a: usize,
        b: usize,
    },
    Neq {
        a: usize,
        b: usize,
    },
    Lt {
        a: usize,
        b: usize,
    },
    Lte {
        a: usize,
        b: usize,
    },
    Gt {
        a: usize,
        b: usize,
    },
    Gte {
        a: usize,
        b: usize,
    },
    BAnd {
        a: usize,
        b: usize,
    },
    BOr {
        a: usize,
        b: usize,
    },
    Neg {
        a: usize,
    },
    BInvert {
        a: usize,
    },
    Min {
        a: usize,
        b: usize,
    },
    Max {
        a: usize,
        b: usize,
    },
    Abs {
        a: usize,
    },
    Inf,
    ToStr {
        a: usize,
    },
    #[serde(rename_all = "camelCase")]
    MeasureTextX {
        text: usize,
        font_size: usize,
    },
    #[serde(rename_all = "camelCase")]
    MeasureTextY {
        text: usize,
        font_size: usize,
    },
}

#[derive(Serialize, Debug, Clone, Hash, Eq, PartialEq)]
#[serde(untagged)]
pub enum ExprPartRef {
    IntLiteral(i64),
    #[serde(with = "F32Def")]
    FloatLiteral(F32),
    StringLiteral(String),
    Operation(ExprOpRef),
}

#[derive(Serialize)]
#[serde(remote = "F32")]
pub struct F32Def(pub f32);

#[derive(Serialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ExprPart {
    IntLiteral(i64),
    FloatLiteral(f32),
    StringLiteral(String),
    Operation(ExprOp),
}

impl ExprOp {
    pub fn to_expr_op_ref(&self, oplist: &mut OperationList) -> ExprOpRef {
        match self {
            ExprOp::Var { name } => ExprOpRef::Var { name: name.clone() },
            ExprOp::Add { a, b } => ExprOpRef::Add {
                a: oplist.add(a),
                b: oplist.add(b),
            },
            ExprOp::Mul { a, b } => ExprOpRef::Mul {
                a: oplist.add(a),
                b: oplist.add(b),
            },
            ExprOp::Div { a, b } => ExprOpRef::Div {
                a: oplist.add(a),
                b: oplist.add(b),
            },
            ExprOp::Fdiv { a, b } => ExprOpRef::Fdiv {
                a: oplist.add(a),
                b: oplist.add(b),
            },
            ExprOp::Mod { a, b } => ExprOpRef::Mod {
                a: oplist.add(a),
                b: oplist.add(b),
            },
            ExprOp::Pow { a, b } => ExprOpRef::Pow {
                a: oplist.add(a),
                b: oplist.add(b),
            },
            ExprOp::Eq { a, b } => ExprOpRef::Eq {
                a: oplist.add(a),
                b: oplist.add(b),
            },
            ExprOp::Neq { a, b } => ExprOpRef::Neq {
                a: oplist.add(a),
                b: oplist.add(b),
            },
            ExprOp::Lt { a, b } => ExprOpRef::Lt {
                a: oplist.add(a),
                b: oplist.add(b),
            },
            ExprOp::Lte { a, b } => ExprOpRef::Lte {
                a: oplist.add(a),
                b: oplist.add(b),
            },
            ExprOp::Gt { a, b } => ExprOpRef::Gt {
                a: oplist.add(a),
                b: oplist.add(b),
            },
            ExprOp::Gte { a, b } => ExprOpRef::Gte {
                a: oplist.add(a),
                b: oplist.add(b),
            },
            ExprOp::BAnd { a, b } => ExprOpRef::BAnd {
                a: oplist.add(a),
                b: oplist.add(b),
            },
            ExprOp::BOr { a, b } => ExprOpRef::BOr {
                a: oplist.add(a),
                b: oplist.add(b),
            },
            ExprOp::Neg { a } => ExprOpRef::Neg { a: oplist.add(a) },
            ExprOp::BInvert { a } => ExprOpRef::BInvert { a: oplist.add(a) },
            ExprOp::Min { a, b } => ExprOpRef::Min {
                a: oplist.add(a),
                b: oplist.add(b),
            },
            ExprOp::Max { a, b } => ExprOpRef::Max {
                a: oplist.add(a),
                b: oplist.add(b),
            },
            ExprOp::Abs { a } => ExprOpRef::Abs { a: oplist.add(a) },
            ExprOp::Inf => ExprOpRef::Inf,
            ExprOp::ToStr { a } => ExprOpRef::ToStr { a: oplist.add(a) },
            ExprOp::MeasureTextX { text, font_size } => ExprOpRef::MeasureTextX {
                text: oplist.add(text),
                font_size: oplist.add(font_size),
            },
            ExprOp::MeasureTextY { text, font_size } => ExprOpRef::MeasureTextY {
                text: oplist.add(text),
                font_size: oplist.add(font_size),
            },
        }
    }

    /// # Safety
    ///
    /// All children must be valid ExprParts created from `simplexp_{var,int,float,string,op}_new`
    pub unsafe fn from_ffi_children(
        op_id: ExprOpId,
        child1: *const ExprPart,
        child2: *const ExprPart,
        _child3: *const ExprPart,
        _child4: *const ExprPart,
        _child5: *const ExprPart,
    ) -> ExprOp {
        fn clone_child(expr: *const ExprPart) -> Arc<ExprPart> {
            unsafe { Arc::clone_from_ptr(expr) }
        }
        match op_id {
            ExprOpId::Add => ExprOp::Add {
                a: clone_child(child1),
                b: clone_child(child2),
            },
            ExprOpId::Mul => ExprOp::Mul {
                a: clone_child(child1),
                b: clone_child(child2),
            },
            ExprOpId::Div => ExprOp::Div {
                a: clone_child(child1),
                b: clone_child(child2),
            },
            ExprOpId::Fdiv => ExprOp::Fdiv {
                a: clone_child(child1),
                b: clone_child(child2),
            },
            ExprOpId::Mod => ExprOp::Mod {
                a: clone_child(child1),
                b: clone_child(child2),
            },
            ExprOpId::Pow => ExprOp::Pow {
                a: clone_child(child1),
                b: clone_child(child2),
            },
            ExprOpId::Eq => ExprOp::Eq {
                a: clone_child(child1),
                b: clone_child(child2),
            },
            ExprOpId::Neq => ExprOp::Neq {
                a: clone_child(child1),
                b: clone_child(child2),
            },
            ExprOpId::Lt => ExprOp::Lt {
                a: clone_child(child1),
                b: clone_child(child2),
            },
            ExprOpId::Lte => ExprOp::Lte {
                a: clone_child(child1),
                b: clone_child(child2),
            },
            ExprOpId::Gt => ExprOp::Gt {
                a: clone_child(child1),
                b: clone_child(child2),
            },
            ExprOpId::Gte => ExprOp::Gte {
                a: clone_child(child1),
                b: clone_child(child2),
            },
            ExprOpId::BAnd => ExprOp::BAnd {
                a: clone_child(child1),
                b: clone_child(child2),
            },
            ExprOpId::BOr => ExprOp::BOr {
                a: clone_child(child1),
                b: clone_child(child2),
            },
            ExprOpId::Neg => ExprOp::Neg {
                a: clone_child(child1),
            },
            ExprOpId::BInvert => ExprOp::BInvert {
                a: clone_child(child1),
            },
            ExprOpId::Min => ExprOp::Min {
                a: clone_child(child1),
                b: clone_child(child2),
            },
            ExprOpId::Max => ExprOp::Max {
                a: clone_child(child1),
                b: clone_child(child2),
            },
            ExprOpId::Abs => ExprOp::Abs {
                a: clone_child(child1),
            },
            ExprOpId::ToStr => ExprOp::ToStr {
                a: clone_child(child1),
            },
            ExprOpId::MeasureTextX => ExprOp::MeasureTextX {
                text: clone_child(child1),
                font_size: clone_child(child2),
            },
            ExprOpId::MeasureTextY => ExprOp::MeasureTextY {
                text: clone_child(child1),
                font_size: clone_child(child2),
            },
        }
    }
}

impl Debug for ExprOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprOp::Var { name } => write!(f, "{}", name),
            ExprOp::Add { a, b } => write!(f, "({:?} + {:?})", a, b),
            ExprOp::Mul { a, b } => write!(f, "({:?} * {:?})", a, b),
            ExprOp::Div { a, b } => write!(f, "({:?} / {:?})", a, b),
            ExprOp::Fdiv { a, b } => write!(f, "({:?} // {:?})", a, b),
            ExprOp::Mod { a, b } => write!(f, "({:?} % {:?})", a, b),
            ExprOp::Pow { a, b } => write!(f, "({:?} ** {:?})", a, b),
            ExprOp::Eq { a, b } => write!(f, "({:?} == {:?})", a, b),
            ExprOp::Neq { a, b } => write!(f, "({:?} != {:?})", a, b),
            ExprOp::Lt { a, b } => write!(f, "({:?} < {:?})", a, b),
            ExprOp::Lte { a, b } => write!(f, "({:?} <= {:?})", a, b),
            ExprOp::Gt { a, b } => write!(f, "({:?} > {:?})", a, b),
            ExprOp::Gte { a, b } => write!(f, "({:?} >= {:?})", a, b),
            ExprOp::BAnd { a, b } => write!(f, "({:?} & {:?})", a, b),
            ExprOp::BOr { a, b } => write!(f, "({:?} | {:?})", a, b),
            ExprOp::Neg { a } => write!(f, "-{:?}", a),
            ExprOp::BInvert { a } => write!(f, "~{:?}", a),
            ExprOp::Min { a, b } => write!(f, "min({:?}, {:?})", a, b),
            ExprOp::Max { a, b } => write!(f, "max({:?}, {:?})", a, b),
            ExprOp::Abs { a } => write!(f, "abs({:?})", a),
            ExprOp::Inf => write!(f, "INF"),
            ExprOp::ToStr { a } => write!(f, "toStr({:?})", a),
            ExprOp::MeasureTextX { text, font_size } => {
                write!(f, "measureTextX(text={:?}, fontSize={:?})", text, font_size)
            }
            ExprOp::MeasureTextY { text, font_size } => {
                write!(f, "measureTextY(text={:?}, fontSize={:?})", text, font_size)
            }
        }
    }
}

impl Debug for ExprPart {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprPart::IntLiteral(val) => write!(f, "{}", val),
            ExprPart::FloatLiteral(val) => write!(f, "{}", val),
            ExprPart::StringLiteral(val) => write!(f, "{:?}", val),
            ExprPart::Operation(val) => write!(f, "{:?}", val),
        }
    }
}
