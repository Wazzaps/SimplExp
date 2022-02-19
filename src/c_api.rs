use crate::expressions::{ExprOp, ExprOpId, ExprPart};
use crate::operation_list::OperationList;
use crate::optimizer;
use konst::{primitive::parse_u32, unwrap_ctx};
use num_traits::FromPrimitive;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::panic::catch_unwind;
use std::ptr::null;
use std::sync::Arc;

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

        Arc::into_raw(optimizer::optimize(Arc::new(expr))) as *const ExprPart
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

/// Serialize an expression into a JSON string.
/// The format is `{(op), "a": {(op), ...}}`
#[no_mangle]
pub extern "C" fn simplexp_serialize_expr(expr: *const ExprPart) -> VecInner {
    catch_unwind(|| {
        let expr = unsafe { (expr as *const ExprPart).as_ref().unwrap() };
        let (ptr, len, cap) = serde_json::to_vec(expr).unwrap().into_raw_parts();
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

// /// Collect operation list, or append to one
// #[no_mangle]
// pub extern "C" fn simplexp_make_oplist(expr: *const ExprPart) -> VecInner {
//     catch_unwind(|| {
//         let expr = unsafe { (expr as *const ExprPart).as_ref().unwrap() };
//         let oplist = OperationList::from(expr);
//         let (ptr, len, cap) = serde_json::to_vec(&oplist).unwrap().into_raw_parts();
//         VecInner {
//             ptr: ptr as *const u8,
//             len,
//             cap,
//         }
//     })
//     .unwrap_or(VecInner {
//         ptr: null(),
//         len: 0,
//         cap: 0,
//     })
// }

/// Serialize an expression into a JSON string.
/// The format is `[{(op), "a": (ref_id)}, ...]`
#[no_mangle]
pub extern "C" fn simplexp_serialize_expr_ops(expr: *const ExprPart) -> VecInner {
    catch_unwind(|| {
        let expr = unsafe { (expr as *const ExprPart).as_ref().unwrap() };
        let oplist = OperationList::from(expr);
        let (ptr, len, cap) = serde_json::to_vec(&oplist).unwrap().into_raw_parts();
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

/// Frees a string allocated by `simplexp_format_expr`, `simplexp_serialize_expr`, or `simplexp_serialize_expr_ops`.
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

/// Creates a copy of the given expression.
#[no_mangle]
pub extern "C" fn simplexp_clone_expr(expr: *const ExprPart) -> *const ExprPart {
    catch_unwind(|| {
        assert!(!expr.is_null());
        unsafe { Arc::into_raw(Arc::clone_from_ptr(expr)) as *const ExprPart }
    })
    .unwrap_or(null())
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
