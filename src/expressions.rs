use eq_float::F32;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

macro_rules! type_to_ref {
    (ExprPart) => {
        usize
    };
    ($type:ty) => {
        $type
    };
}

macro_rules! type_to_realtype {
    (ExprPart) => {
        Arc<ExprPart>
    };
    ($type:ty) => {
        $type
    };
}

macro_rules! define_ops {
    (
        $($name:ident: {
            id: $id:literal,
            fields: {
                $($field:ident: $field_type:ident = $child_id:tt,)*
            },
            format: $format:literal,
        }),*
    ) => {
        use num_derive::FromPrimitive;
        use serde::Serialize;

        #[derive(FromPrimitive)]
        pub enum ExprOpId {
            $($name = $id,)*
        }

        #[derive(Serialize, Clone, PartialEq)]
        #[serde(tag = "type", rename_all = "camelCase")]
        pub enum ExprOp {
            #[serde(rename_all = "camelCase")]
            Var { name: String },
            Inf,
            $(
                #[serde(rename_all = "camelCase")]
                $name {
                    $($field: type_to_realtype!($field_type),)*
                },
            )*
        }

        #[derive(Serialize, Debug, Clone, Hash, Eq, PartialEq)]
        #[serde(tag = "type", rename_all = "camelCase")]
        pub enum ExprOpRef {
            #[serde(rename_all = "camelCase")]
            Var { name: String },
            Inf,
            $(
                #[serde(rename_all = "camelCase")]
                $name {
                    $($field: type_to_ref!($field_type),)*
                },
            )*
        }

        impl ExprOp {
            pub fn to_expr_op_ref(&self, oplist: &mut crate::operation_list::OperationList) -> ExprOpRef {
                match self {
                    ExprOp::Var { name } => ExprOpRef::Var { name: name.clone() },
                    ExprOp::Inf => ExprOpRef::Inf,
                    $(
                        ExprOp::$name { $($field,)* } => ExprOpRef::$name {
                            $($field: oplist.add($field),)*
                        },
                    )*
                }
            }

            /// # Safety
            ///
            /// All children must be valid ExprParts created from `simplexp_{var,int,float,string,op}_new`
            #[allow(unused_variables)]
            pub unsafe fn from_ffi_children(
                op_id: ExprOpId,
                child1: *const ExprPart,
                child2: *const ExprPart,
                child3: *const ExprPart,
                child4: *const ExprPart,
                child5: *const ExprPart,
            ) -> ExprOp {
                fn clone_child(expr: *const ExprPart) -> Arc<ExprPart> {
                    use $crate::c_api::CloneFromPtr;
                    unsafe { Arc::clone_from_ptr(expr) }
                }
                match op_id {
                    $(
                        ExprOpId::$name => ExprOp::$name {
                            $($field: clone_child(concat_idents!($child_id)),)*
                        },
                    )*
                }
            }
        }

        impl core::fmt::Debug for ExprOp {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    ExprOp::Var { name } => write!(f, "{name}"),
                    ExprOp::Inf => write!(f, "INF"),
                    $(
                        // #[serde(rename_all = "camelCase")]
                        ExprOp::$name { $($field,)* } => {
                            write!(f, $format)
                        },
                    )*
                }
            }
        }
    }
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

define_ops! {
    Add: {
        id: 1,
        fields: {
            a: ExprPart = child1,
            b: ExprPart = child2,
        },
        format: "({a:?} + {b:?})",
    },
    Mul: {
        id: 2,
        fields: {
            a: ExprPart = child1,
            b: ExprPart = child2,
        },
        format: "({a:?} * {b:?})",
    },
    Div: {
        id: 3,
        fields: {
            a: ExprPart = child1,
            b: ExprPart = child2,
        },
        format: "({a:?} / {b:?})",
    },
    Fdiv: {
        id: 4,
        fields: {
            a: ExprPart = child1,
            b: ExprPart = child2,
        },
        format: "({a:?} // {b:?})",
    },
    Mod: {
        id: 5,
        fields: {
            a: ExprPart = child1,
            b: ExprPart = child2,
        },
        format: "({a:?} % {b:?})",
    },
    Pow: {
        id: 6,
        fields: {
            a: ExprPart = child1,
            b: ExprPart = child2,
        },
        format: "({a:?} ** {b:?})",
    },
    Eq: {
        id: 7,
        fields: {
            a: ExprPart = child1,
            b: ExprPart = child2,
        },
        format: "({a:?} == {b:?})",
    },
    Neq: {
        id: 8,
        fields: {
            a: ExprPart = child1,
            b: ExprPart = child2,
        },
        format: "({a:?} != {b:?})",
    },
    Lt: {
        id: 9,
        fields: {
            a: ExprPart = child1,
            b: ExprPart = child2,
        },
        format: "({a:?} < {b:?})",
    },
    Lte: {
        id: 10,
        fields: {
            a: ExprPart = child1,
            b: ExprPart = child2,
        },
        format: "({a:?} <= {b:?})",
    },
    Gt: {
        id: 11,
        fields: {
            a: ExprPart = child1,
            b: ExprPart = child2,
        },
        format: "({a:?} > {b:?})",
    },
    Gte: {
        id: 12,
        fields: {
            a: ExprPart = child1,
            b: ExprPart = child2,
        },
        format: "({a:?} >= {b:?})",
    },
    BAnd: {
        id: 13,
        fields: {
            a: ExprPart = child1,
            b: ExprPart = child2,
        },
        format: "({a:?} & {b:?})",
    },
    BOr: {
        id: 14,
        fields: {
            a: ExprPart = child1,
            b: ExprPart = child2,
        },
        format: "({a:?} | {b:?})",
    },
    Neg: {
        id: 15,
        fields: {
            a: ExprPart = child1,
        },
        format: "-{a:?}",
    },
    BInvert: {
        id: 16,
        fields: {
            a: ExprPart = child1,
        },
        format: "~{a:?}",
    },
    Min: {
        id: 17,
        fields: {
            a: ExprPart = child1,
            b: ExprPart = child2,
        },
        format: "min({a:?}, {b:?})",
    },
    Max: {
        id: 18,
        fields: {
            a: ExprPart = child1,
            b: ExprPart = child2,
        },
        format: "max({a:?}, {b:?})",
    },
    Abs: {
        id: 19,
        fields: {
            a: ExprPart = child1,
        },
        format: "abs({a:?})",
    },
    ToStr: {
        id: 20,
        fields: {
            a: ExprPart = child1,
        },
        format: "toStr({a:?})",
    },
    MeasureTextX: {
        id: 21,
        fields: {
            text: ExprPart = child1,
            font_size: ExprPart = child2,
        },
        format: "measureTextX(text={text:?}, fontSize={font_size:?})",
    },
    MeasureTextY: {
        id: 22,
        fields: {
            text: ExprPart = child1,
            font_size: ExprPart = child2,
        },
        format: "measureTextY(text={text:?}, fontSize={font_size:?})",
    },
    If: {
        id: 23,
        fields: {
            cond: ExprPart = child1,
            t: ExprPart = child2,
            f: ExprPart = child3,
        },
        format: "if ({cond:?}) {{{t:?}}} else {{{f:?}}}",
    }
}
