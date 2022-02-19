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
        a: ExprPartRef,
        b: ExprPartRef,
    },
    Mul {
        a: ExprPartRef,
        b: ExprPartRef,
    },
    Div {
        a: ExprPartRef,
        b: ExprPartRef,
    },
    Fdiv {
        a: ExprPartRef,
        b: ExprPartRef,
    },
    Mod {
        a: ExprPartRef,
        b: ExprPartRef,
    },
    Pow {
        a: ExprPartRef,
        b: ExprPartRef,
    },
    Eq {
        a: ExprPartRef,
        b: ExprPartRef,
    },
    Neq {
        a: ExprPartRef,
        b: ExprPartRef,
    },
    Lt {
        a: ExprPartRef,
        b: ExprPartRef,
    },
    Lte {
        a: ExprPartRef,
        b: ExprPartRef,
    },
    Gt {
        a: ExprPartRef,
        b: ExprPartRef,
    },
    Gte {
        a: ExprPartRef,
        b: ExprPartRef,
    },
    BAnd {
        a: ExprPartRef,
        b: ExprPartRef,
    },
    BOr {
        a: ExprPartRef,
        b: ExprPartRef,
    },
    Neg {
        a: ExprPartRef,
    },
    BInvert {
        a: ExprPartRef,
    },
    Min {
        a: ExprPartRef,
        b: ExprPartRef,
    },
    Max {
        a: ExprPartRef,
        b: ExprPartRef,
    },
    Abs {
        a: ExprPartRef,
    },
    Inf,
    ToStr {
        a: ExprPartRef,
    },
    #[serde(rename_all = "camelCase")]
    MeasureTextX {
        text: ExprPartRef,
        font_size: ExprPartRef,
    },
    #[serde(rename_all = "camelCase")]
    MeasureTextY {
        text: ExprPartRef,
        font_size: ExprPartRef,
    },
}

#[derive(Serialize, Debug, Clone, Hash, Eq, PartialEq)]
#[serde(untagged)]
pub enum ExprPartRef {
    IntLiteral(i64),
    #[serde(with = "F32Def")]
    FloatLiteral(F32),
    StringLiteral(String),
    Operation(usize),
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
