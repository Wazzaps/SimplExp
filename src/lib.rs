#![feature(vec_into_raw_parts)]

use konst::{primitive::parse_u32, unwrap_ctx};
use std::ffi::CStr;
use std::fmt::{Debug, Formatter};
use std::os::raw::c_char;
use std::panic::catch_unwind;
use std::ptr::null;

#[non_exhaustive]
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

pub enum ExprOp {
    Var {
        name: String,
    },
    Add {
        a: Box<ExprPart>,
        b: Box<ExprPart>,
    },
    Mul {
        a: Box<ExprPart>,
        b: Box<ExprPart>,
    },
    Div {
        a: Box<ExprPart>,
        b: Box<ExprPart>,
    },
    Fdiv {
        a: Box<ExprPart>,
        b: Box<ExprPart>,
    },
    Mod {
        a: Box<ExprPart>,
        b: Box<ExprPart>,
    },
    Pow {
        a: Box<ExprPart>,
        b: Box<ExprPart>,
    },
    Eq {
        a: Box<ExprPart>,
        b: Box<ExprPart>,
    },
    Neq {
        a: Box<ExprPart>,
        b: Box<ExprPart>,
    },
    Lt {
        a: Box<ExprPart>,
        b: Box<ExprPart>,
    },
    Lte {
        a: Box<ExprPart>,
        b: Box<ExprPart>,
    },
    Gt {
        a: Box<ExprPart>,
        b: Box<ExprPart>,
    },
    Gte {
        a: Box<ExprPart>,
        b: Box<ExprPart>,
    },
    BAnd {
        a: Box<ExprPart>,
        b: Box<ExprPart>,
    },
    BOr {
        a: Box<ExprPart>,
        b: Box<ExprPart>,
    },
    Not {
        a: Box<ExprPart>,
    },
    Neg {
        a: Box<ExprPart>,
    },
    BInvert {
        a: Box<ExprPart>,
    },
    Min {
        a: Box<ExprPart>,
        b: Box<ExprPart>,
    },
    Max {
        a: Box<ExprPart>,
        b: Box<ExprPart>,
    },
    Abs {
        a: Box<ExprPart>,
    },
    Inf,
    ToStr {
        a: Box<ExprPart>,
    },
    // #[serde(rename_all = "camelCase")]
    MeasureTextX {
        text: Box<ExprPart>,
        font_size: Box<ExprPart>,
    },
    // #[serde(rename_all = "camelCase")]
    MeasureTextY {
        text: Box<ExprPart>,
        font_size: Box<ExprPart>,
    },
}

// #[derive(Deserialize, Debug)]
// #[serde(untagged)]
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

        Box::into_raw(Box::new(ExprPart::Operation(ExprOp::Var {
            name: name.to_string(),
        }))) as *const ExprPart
    })
    .unwrap_or(null())
}

/// Creates a new float (f32) literal
#[no_mangle]
pub extern "C" fn simplexp_new_float(value: f32) -> *const ExprPart {
    catch_unwind(|| {
        assert!(!value.is_nan());

        let expr = if f32::is_finite(value) {
            ExprPart::FloatLiteral(value)
        } else {
            ExprPart::Operation(ExprOp::Inf)
        };

        Box::into_raw(Box::new(expr)) as *const ExprPart
    })
    .unwrap_or(null())
}

/// Creates a new int (i64) literal.
#[no_mangle]
pub extern "C" fn simplexp_new_int(value: i64) -> *const ExprPart {
    catch_unwind(|| Box::into_raw(Box::new(ExprPart::IntLiteral(value))) as *const ExprPart)
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
        Box::into_raw(Box::new(ExprPart::StringLiteral(string.to_string()))) as *const ExprPart
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
pub extern "C" fn simplexp_format_expr(expr: *mut ExprPart) -> VecInner {
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
        drop(unsafe { Vec::from_raw_parts(inner.ptr as *mut u8, inner.len, inner.cap) });
    });
}

/// Frees an expression allocated by the `simplexp_new_*` functions.
#[no_mangle]
pub extern "C" fn simplexp_free_expr(expr: *mut ExprPart) {
    let _ = catch_unwind(|| {
        if expr.is_null() {
            return;
        }
        drop(unsafe { Box::from_raw(expr) });
    });
}
