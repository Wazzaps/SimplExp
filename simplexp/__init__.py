from __future__ import annotations
from .simplexp import lib as _lib, ffi as _ffi
import math

LIB_VERSION = (_lib.SIMPLEXP_VERSION_MAJOR, _lib.SIMPLEXP_VERSION_MINOR, _lib.SIMPLEXP_VERSION_PATCH)


class Expr:
    def __init__(self, value: Expr | int | float | str | _ffi.CData):
        if isinstance(value, _ffi.CData):
            self._inner = value
        elif isinstance(value, Expr):
            self._inner = _lib.simplexp_clone_expr(value._inner)
        elif value is None:
            raise ValueError('Cannot encode null values')
        elif isinstance(value, float):
            if math.isnan(value):
                raise ValueError('Cannot encode NaN values')
            else:
                self._inner = _lib.simplexp_new_float(value)
        elif isinstance(value, int):
            self._inner = _lib.simplexp_new_int(value)
        elif isinstance(value, str):
            value = value.encode('utf8')
            self._inner = _lib.simplexp_new_str(_ffi.from_buffer(value), len(value))

        assert self._inner, 'Failed to create expression'

    def __str__(self):
        vec = _lib.simplexp_format_expr(self._inner)
        formatted = str(_ffi.buffer(vec.ptr, vec.len), 'utf8')
        _lib.simplexp_free_str(vec)
        return formatted

    def __del__(self):
        _lib.simplexp_free_expr(self._inner)


def var(name: str):
    return Expr(_lib.simplexp_new_var(bytes(name, 'utf-8')))
