#![feature(vec_into_raw_parts)]

mod optimizer;

use konst::{primitive::parse_u32, unwrap_ctx};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::ffi::CStr;
use std::fmt::{Debug, Formatter};
use std::os::raw::c_char;
use std::panic::catch_unwind;
use std::ptr::null;
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
    Not = 15,
    Neg = 16,
    BInvert = 17,
    Min = 18,
    Max = 19,
    Abs = 20,
    ToStr = 21,
    MeasureTextX = 22,
    MeasureTextY = 23,
}

#[derive(Clone, PartialEq)]
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
    Not {
        a: Arc<ExprPart>,
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
    // #[serde(rename_all = "camelCase")]
    MeasureTextX {
        text: Arc<ExprPart>,
        font_size: Arc<ExprPart>,
    },
    // #[serde(rename_all = "camelCase")]
    MeasureTextY {
        text: Arc<ExprPart>,
        font_size: Arc<ExprPart>,
    },
}

// #[derive(Deserialize, Debug)]
// #[serde(untagged)]
#[derive(Clone, PartialEq)]
pub enum ExprPart {
    IntLiteral(i64),
    FloatLiteral(f32),
    StringLiteral(String),
    Operation(ExprOp),
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
            ExprOp::Not { a } => write!(f, "!{:?}", a),
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

#[no_mangle]
pub static SIMPLEXP_VERSION_MAJOR: u32 = unwrap_ctx!(parse_u32(env!("CARGO_PKG_VERSION_MAJOR")));
#[no_mangle]
pub static SIMPLEXP_VERSION_MINOR: u32 = unwrap_ctx!(parse_u32(env!("CARGO_PKG_VERSION_MINOR")));
#[no_mangle]
pub static SIMPLEXP_VERSION_PATCH: u32 = unwrap_ctx!(parse_u32(env!("CARGO_PKG_VERSION_PATCH")));

/// Creates a new variable binding.
#[no_mangle]
pub extern "C" fn simplexp_new_var(name: *const c_char) -> *const ExprPart {
    catch_unwind(|| {
        let name: &str = std::str::from_utf8(unsafe { CStr::from_ptr(name).to_bytes() }).unwrap();

        Arc::into_raw(Arc::new(ExprPart::Operation(ExprOp::Var {
            name: name.to_string(),
        }))) as *const ExprPart
    })
    .unwrap_or(null())
}

/// Wraps the expression(s) in an operation.
#[no_mangle]
pub extern "C" fn simplexp_new_op(
    op_id: i32,
    child1: *const ExprPart,
    child2: *const ExprPart,
    _child3: *const ExprPart,
    _child4: *const ExprPart,
    _child5: *const ExprPart,
) -> *const ExprPart {
    catch_unwind(|| {
        fn clone_child(expr: *const ExprPart) -> Arc<ExprPart> {
            unsafe { Arc::clone_from_ptr(expr) }
        }
        let op_id: ExprOpId = FromPrimitive::from_i32(op_id).unwrap();
        let expr = ExprPart::Operation(match op_id {
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
            ExprOpId::Not => ExprOp::Not {
                a: clone_child(child1),
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
        });

        Arc::into_raw(Arc::new(expr)) as *const ExprPart
    })
    .unwrap_or(null())
}

/// Creates a simplified expression from the given expression.
#[no_mangle]
pub extern "C" fn simplexp_simplify(expr: *const ExprPart) -> *const ExprPart {
    catch_unwind(|| {
        assert!(!expr.is_null());
        let prev_expr = unsafe { Arc::clone_from_ptr(expr) };
        let optimized = optimizer::optimize(prev_expr);
        Arc::into_raw(optimized) as *const ExprPart
    })
    .unwrap_or(null())
}

/// Creates a new float (f32) literal.
#[no_mangle]
pub extern "C" fn simplexp_new_float(value: f32) -> *const ExprPart {
    catch_unwind(|| {
        assert!(!value.is_nan());

        let expr = if f32::is_finite(value) {
            ExprPart::FloatLiteral(value)
        } else {
            ExprPart::Operation(ExprOp::Inf)
        };

        Arc::into_raw(Arc::new(expr)) as *const ExprPart
    })
    .unwrap_or(null())
}

/// Creates a new int (i64) literal.
#[no_mangle]
pub extern "C" fn simplexp_new_int(value: i64) -> *const ExprPart {
    catch_unwind(|| Arc::into_raw(Arc::new(ExprPart::IntLiteral(value))) as *const ExprPart)
        .unwrap_or(null())
}

/// Creates a new string literal.
#[no_mangle]
pub extern "C" fn simplexp_new_str(value: *const u8, length: usize) -> *const ExprPart {
    catch_unwind(|| {
        if value.is_null() {
            return null();
        }
        let string =
            unsafe { std::str::from_utf8(std::slice::from_raw_parts(value, length)) }.unwrap();
        Arc::into_raw(Arc::new(ExprPart::StringLiteral(string.to_string()))) as *const ExprPart
    })
    .unwrap_or(null())
}

/// Used to pass the string to ffi consumers
#[repr(C)]
pub struct VecInner {
    ptr: *const u8,
    len: usize,
    cap: usize,
}

/// Formats an expression into a string.
#[no_mangle]
pub extern "C" fn simplexp_format_expr(expr: *const ExprPart) -> VecInner {
    catch_unwind(|| {
        let expr = unsafe { (expr as *const ExprPart).as_ref().unwrap() };
        let (ptr, len, cap) = format!("{:?}", expr).into_bytes().into_raw_parts();
        VecInner {
            ptr: ptr as *const u8,
            len,
            cap,
        }
    })
    .unwrap_or(VecInner {
        ptr: null(),
        len: 0,
        cap: 0,
    })
}

/// Frees a string allocated by `simplexp_format_expr`.
#[no_mangle]
pub extern "C" fn simplexp_free_str(inner: VecInner) {
    let _ = catch_unwind(|| {
        if inner.ptr.is_null() {
            return;
        }
        let _ = unsafe { Vec::from_raw_parts(inner.ptr as *mut u8, inner.len, inner.cap) };
    });
}

/// Frees an expression allocated by the `simplexp_new_*` functions.
#[no_mangle]
pub extern "C" fn simplexp_free_expr(expr: *const ExprPart) {
    let _ = catch_unwind(|| {
        assert!(!expr.is_null());
        let _ = unsafe { Arc::from_raw(expr as *mut ExprPart) };
    });
}

trait CloneFromPtr<T> {
    unsafe fn clone_from_ptr(ptr: *const T) -> Self;
}

impl<T> CloneFromPtr<T> for Arc<T> {
    unsafe fn clone_from_ptr(ptr: *const T) -> Self {
        assert!(!ptr.is_null());
        let original = Arc::from_raw(ptr);
        let copy = original.clone();
        let _ = Arc::into_raw(original);
        copy
    }
}
