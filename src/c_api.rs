use crate::expressions::{ExprOp, ExprOpId, ExprPart};
use crate::operation_list::OperationList;
use crate::optimizer;
use konst::{primitive::parse_u32, unwrap_ctx};
use num_traits::FromPrimitive;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::panic::catch_unwind;
use std::ptr::null;
use std::sync::{Arc, Mutex};

#[no_mangle]
pub static SIMPLEXP_VERSION_MAJOR: u32 = unwrap_ctx!(parse_u32(env!("CARGO_PKG_VERSION_MAJOR")));
#[no_mangle]
pub static SIMPLEXP_VERSION_MINOR: u32 = unwrap_ctx!(parse_u32(env!("CARGO_PKG_VERSION_MINOR")));
#[no_mangle]
pub static SIMPLEXP_VERSION_PATCH: u32 = unwrap_ctx!(parse_u32(env!("CARGO_PKG_VERSION_PATCH")));

/// Creates a new variable binding.
#[no_mangle]
pub extern "C" fn simplexp_var_new(name: *const c_char) -> *const ExprPart {
    catch_unwind(|| {
        let name: &str = std::str::from_utf8(unsafe { CStr::from_ptr(name).to_bytes() }).unwrap();

        Arc::into_raw(Arc::new(ExprPart::Operation(ExprOp::Var {
            name: name.to_string(),
        })))
    })
    .unwrap_or(null())
}

/// Wraps the expression(s) in an operation.
#[no_mangle]
pub extern "C" fn simplexp_op_new(
    op_id: i32,
    child1: *const ExprPart,
    child2: *const ExprPart,
    child3: *const ExprPart,
    child4: *const ExprPart,
    child5: *const ExprPart,
) -> *const ExprPart {
    catch_unwind(|| {
        let op_id: ExprOpId = FromPrimitive::from_i32(op_id).unwrap();
        let expr = ExprPart::Operation(unsafe {
            ExprOp::from_ffi_children(op_id, child1, child2, child3, child4, child5)
        });

        Arc::into_raw(optimizer::optimize(Arc::new(expr)))
    })
    .unwrap_or(null())
}

/// Creates a new float (f64) literal.
#[no_mangle]
pub extern "C" fn simplexp_float_new(value: f64) -> *const ExprPart {
    catch_unwind(|| {
        assert!(!value.is_nan());

        let expr = if f64::is_finite(value) {
            ExprPart::FloatLiteral(value)
        } else {
            ExprPart::Operation(ExprOp::Inf)
        };

        Arc::into_raw(Arc::new(expr))
    })
    .unwrap_or(null())
}

/// Creates a new int (i64) literal.
#[no_mangle]
pub extern "C" fn simplexp_int_new(value: i64) -> *const ExprPart {
    catch_unwind(|| Arc::into_raw(Arc::new(ExprPart::IntLiteral(value)))).unwrap_or(null())
}

/// Creates a new string literal.
#[no_mangle]
pub extern "C" fn simplexp_str_new(value: *const u8, length: usize) -> *const ExprPart {
    catch_unwind(|| {
        if value.is_null() {
            return null();
        }
        let string =
            unsafe { std::str::from_utf8(std::slice::from_raw_parts(value, length)) }.unwrap();
        Arc::into_raw(Arc::new(ExprPart::StringLiteral(string.to_string())))
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
pub extern "C" fn simplexp_expr_format(expr: *const ExprPart) -> VecInner {
    catch_unwind(|| {
        let expr = unsafe { expr.as_ref().unwrap() };
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
pub extern "C" fn simplexp_expr_serialize(expr: *const ExprPart) -> VecInner {
    catch_unwind(|| {
        let expr = unsafe { expr.as_ref().unwrap() };
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

/// Creates a new operation list.
#[no_mangle]
pub extern "C" fn simplexp_oplist_new() -> *const () {
    catch_unwind(|| Arc::into_raw(Arc::new(Mutex::new(OperationList::new()))) as *const ())
        .unwrap_or(null())
}

/// Append an expression to an operation list, returning the expression's id in the oplist.
#[no_mangle]
pub extern "C" fn simplexp_oplist_append(oplist: *const (), expr: *const ExprPart) -> usize {
    catch_unwind(|| {
        let expr = unsafe { expr.as_ref().unwrap() };
        let oplist = unsafe { (oplist as *const Mutex<OperationList>).as_ref().unwrap() };

        oplist.lock().unwrap().add(expr)
    })
    .unwrap_or(usize::MAX)
}

/// Serialize an oplist into a JSON string.
/// The format is `[{(op), "a": (ref_id)}, ...]`
#[no_mangle]
pub extern "C" fn simplexp_oplist_serialize(oplist: *const ()) -> VecInner {
    catch_unwind(|| {
        let oplist = unsafe { Arc::from_raw(oplist as *const Mutex<OperationList>) };
        let (ptr, len, cap) = serde_json::to_vec(&*oplist.lock().unwrap())
            .unwrap()
            .into_raw_parts();
        let _ = Arc::into_raw(oplist);
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

/// Frees a string allocated by `simplexp_expr_format`, `simplexp_expr_serialize`, or `simplexp_oplist_serialize`.
#[no_mangle]
pub extern "C" fn simplexp_str_free(inner: VecInner) {
    let _ = catch_unwind(|| {
        if inner.ptr.is_null() {
            return;
        }
        let _ = unsafe { Vec::from_raw_parts(inner.ptr as *mut u8, inner.len, inner.cap) };
    });
}

/// Frees an expression allocated by the `simplexp_{var,int,float,string,op}_new` functions.
#[no_mangle]
pub extern "C" fn simplexp_expr_free(expr: *const ExprPart) {
    let _ = catch_unwind(|| {
        assert!(!expr.is_null());
        let _ = unsafe { Arc::from_raw(expr) };
    });
}

/// Creates a copy of the given expression.
#[no_mangle]
pub extern "C" fn simplexp_expr_clone(expr: *const ExprPart) -> *const ExprPart {
    catch_unwind(|| {
        assert!(!expr.is_null());
        unsafe { Arc::into_raw(Arc::clone_from_ptr(expr)) }
    })
    .unwrap_or(null())
}

/// Frees an oplist allocated by the `simplexp_oplist_new` function.
#[no_mangle]
pub extern "C" fn simplexp_oplist_free(oplist: *const ()) {
    let _ = catch_unwind(|| {
        assert!(!oplist.is_null());
        let _ = unsafe { Arc::from_raw(oplist as *const Mutex<OperationList>) };
    });
}

pub(crate) trait CloneFromPtr<T> {
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
