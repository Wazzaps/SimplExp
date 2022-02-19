from __future__ import annotations
from .simplexp import lib as _lib, ffi as _ffi
import json
import math

LIB_VERSION = (_lib.SIMPLEXP_VERSION_MAJOR, _lib.SIMPLEXP_VERSION_MINOR, _lib.SIMPLEXP_VERSION_PATCH)

ExprOpId_Add = 1
ExprOpId_Mul = 2
ExprOpId_Div = 3
ExprOpId_Fdiv = 4
ExprOpId_Mod = 5
ExprOpId_Pow = 6
ExprOpId_Eq = 7
ExprOpId_Neq = 8
ExprOpId_Lt = 9
ExprOpId_Lte = 10
ExprOpId_Gt = 11
ExprOpId_Gte = 12
ExprOpId_BAnd = 13
ExprOpId_BOr = 14
ExprOpId_Neg = 15
ExprOpId_BInvert = 16
ExprOpId_Min = 17
ExprOpId_Max = 18
ExprOpId_Abs = 19
ExprOpId_ToStr = 20
ExprOpId_MeasureTextX = 21
ExprOpId_MeasureTextY = 22


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

    @staticmethod
    def wrap(expr: Expr | int | float | str):
        if isinstance(expr, Expr):
            return expr
        else:
            return Expr(expr)

    def min(self, other: Expr | int | float | str):
        other = Expr.wrap(other)
        return Expr(_lib.simplexp_new_op(
            ExprOpId_Min, self._inner, other._inner, _ffi.NULL, _ffi.NULL, _ffi.NULL
        ))

    def max(self, other: Expr | int | float | str):
        other = Expr.wrap(other)
        return Expr(_lib.simplexp_new_op(
            ExprOpId_Max, self._inner, other._inner, _ffi.NULL, _ffi.NULL, _ffi.NULL
        ))

    def __add__(self, other: Expr | int | float | str):
        other = Expr.wrap(other)
        return Expr(_lib.simplexp_new_op(
            ExprOpId_Add, self._inner, other._inner, _ffi.NULL, _ffi.NULL, _ffi.NULL
        ))

    def __sub__(self, other: Expr | int | float | str):
        other = -Expr.wrap(other)
        return Expr(_lib.simplexp_new_op(
            ExprOpId_Add, self._inner, other._inner, _ffi.NULL, _ffi.NULL, _ffi.NULL
        ))

    def __mul__(self, other: Expr | int | float | str):
        other = Expr.wrap(other)
        return Expr(_lib.simplexp_new_op(
            ExprOpId_Mul, self._inner, other._inner, _ffi.NULL, _ffi.NULL, _ffi.NULL
        ))

    def __mod__(self, other: Expr | int | float | str):
        other = Expr.wrap(other)
        return Expr(_lib.simplexp_new_op(
            ExprOpId_Mod, self._inner, other._inner, _ffi.NULL, _ffi.NULL, _ffi.NULL
        ))

    def __truediv__(self, other: Expr | int | float | str):
        other = Expr.wrap(other)
        return Expr(_lib.simplexp_new_op(
            ExprOpId_Div, self._inner, other._inner, _ffi.NULL, _ffi.NULL, _ffi.NULL
        ))

    def __floordiv__(self, other: Expr | int | float | str):
        other = Expr.wrap(other)
        return Expr(_lib.simplexp_new_op(
            ExprOpId_Fdiv, self._inner, other._inner, _ffi.NULL, _ffi.NULL, _ffi.NULL
        ))

    def __or__(self, other: Expr | int | float | str):
        other = Expr.wrap(other)
        return Expr(_lib.simplexp_new_op(
            ExprOpId_BOr, self._inner, other._inner, _ffi.NULL, _ffi.NULL, _ffi.NULL
        ))

    def __and__(self, other: Expr | int | float | str):
        other = Expr.wrap(other)
        return Expr(_lib.simplexp_new_op(
            ExprOpId_BAnd, self._inner, other._inner, _ffi.NULL, _ffi.NULL, _ffi.NULL
        ))

    def __gt__(self, other: Expr | int | float | str):
        other = Expr.wrap(other)
        return Expr(_lib.simplexp_new_op(
            ExprOpId_Gt, self._inner, other._inner, _ffi.NULL, _ffi.NULL, _ffi.NULL
        ))

    def __ge__(self, other: Expr | int | float | str):
        other = Expr.wrap(other)
        return Expr(_lib.simplexp_new_op(
            ExprOpId_Gte, self._inner, other._inner, _ffi.NULL, _ffi.NULL, _ffi.NULL
        ))

    def __lt__(self, other: Expr | int | float | str):
        other = Expr.wrap(other)
        return Expr(_lib.simplexp_new_op(
            ExprOpId_Lt, self._inner, other._inner, _ffi.NULL, _ffi.NULL, _ffi.NULL
        ))

    def __le__(self, other: Expr | int | float | str):
        other = Expr.wrap(other)
        return Expr(_lib.simplexp_new_op(
            ExprOpId_Lte, self._inner, other._inner, _ffi.NULL, _ffi.NULL, _ffi.NULL
        ))

    def __eq__(self, other: Expr | int | float | str):
        other = Expr.wrap(other)
        return Expr(_lib.simplexp_new_op(
            ExprOpId_Eq, self._inner, other._inner, _ffi.NULL, _ffi.NULL, _ffi.NULL
        ))

    def __ne__(self, other: Expr | int | float | str):
        other = Expr.wrap(other)
        return Expr(_lib.simplexp_new_op(
            ExprOpId_Neq, self._inner, other._inner, _ffi.NULL, _ffi.NULL, _ffi.NULL
        ))

    def __neg__(self):
        return Expr(_lib.simplexp_new_op(
            ExprOpId_Neg, self._inner, _ffi.NULL, _ffi.NULL, _ffi.NULL, _ffi.NULL
        ))

    def __invert__(self):
        return Expr(_lib.simplexp_new_op(
            ExprOpId_BInvert, self._inner, _ffi.NULL, _ffi.NULL, _ffi.NULL, _ffi.NULL
        ))

    def __abs__(self):
        return Expr(_lib.simplexp_new_op(
            ExprOpId_Abs, self._inner, _ffi.NULL, _ffi.NULL, _ffi.NULL, _ffi.NULL
        ))

    def to_str(self):
        return Expr(_lib.simplexp_new_op(
            ExprOpId_ToStr, self._inner, _ffi.NULL, _ffi.NULL, _ffi.NULL, _ffi.NULL
        ))

    @staticmethod
    def measure_text_x(text: Expr | str, font_size: Expr | int | float):
        text = Expr.wrap(text)
        font_size = Expr.wrap(font_size)
        return Expr(_lib.simplexp_new_op(
            ExprOpId_MeasureTextX, text._inner, font_size._inner, _ffi.NULL, _ffi.NULL, _ffi.NULL
        ))

    @staticmethod
    def measure_text_y(text: Expr | str, font_size: Expr | int | float):
        text = Expr.wrap(text)
        font_size = Expr.wrap(font_size)
        return Expr(_lib.simplexp_new_op(
            ExprOpId_MeasureTextY, text._inner, font_size._inner, _ffi.NULL, _ffi.NULL, _ffi.NULL
        ))

    def __str__(self):
        vec = _lib.simplexp_format_expr(self._inner)
        formatted = str(_ffi.buffer(vec.ptr, vec.len)[:], 'utf8')
        _lib.simplexp_free_str(vec)
        return formatted

    def __repr__(self):
        return str(self)

    def __del__(self):
        if self._inner:
            _lib.simplexp_free_expr(self._inner)

    @staticmethod
    def from_dict(obj: dict):
        if isinstance(obj, dict):
            if obj['type'] == 'var':
                return var(obj['name'])
            elif obj['type'] == 'add':
                return Expr.from_dict(obj['a']) + Expr.from_dict(obj['b'])
            elif obj['type'] == 'sub':
                return Expr.from_dict(obj['a']) - Expr.from_dict(obj['b'])
            elif obj['type'] == 'mul':
                return Expr.from_dict(obj['a']) * Expr.from_dict(obj['b'])
            elif obj['type'] == 'div':
                return Expr.from_dict(obj['a']) / Expr.from_dict(obj['b'])
            elif obj['type'] == 'fdiv':
                return Expr.from_dict(obj['a']) // Expr.from_dict(obj['b'])
            elif obj['type'] == 'mod':
                return Expr.from_dict(obj['a']) % Expr.from_dict(obj['b'])
            elif obj['type'] == 'pow':
                return Expr.from_dict(obj['a']) ** Expr.from_dict(obj['b'])
            elif obj['type'] == 'eq':
                return Expr.from_dict(obj['a']) == Expr.from_dict(obj['b'])
            elif obj['type'] == 'neq':
                return Expr.from_dict(obj['a']) != Expr.from_dict(obj['b'])
            elif obj['type'] == 'lt':
                return Expr.from_dict(obj['a']) < Expr.from_dict(obj['b'])
            elif obj['type'] == 'lte':
                return Expr.from_dict(obj['a']) <= Expr.from_dict(obj['b'])
            elif obj['type'] == 'gt':
                return Expr.from_dict(obj['a']) > Expr.from_dict(obj['b'])
            elif obj['type'] == 'gte':
                return Expr.from_dict(obj['a']) >= Expr.from_dict(obj['b'])
            elif obj['type'] == 'bAnd':
                return Expr.from_dict(obj['a']) & Expr.from_dict(obj['b'])
            elif obj['type'] == 'bOr':
                return Expr.from_dict(obj['a']) | Expr.from_dict(obj['b'])
            elif obj['type'] == 'neg':
                return -Expr.from_dict(obj['a'])
            elif obj['type'] == 'bInvert':
                return ~Expr.from_dict(obj['a'])
            elif obj['type'] == 'min':
                return Expr.from_dict(obj['a']).min(Expr.from_dict(obj['b']))
            elif obj['type'] == 'max':
                return Expr.from_dict(obj['a']).max(Expr.from_dict(obj['b']))
            elif obj['type'] == 'abs':
                return abs(Expr.from_dict(obj['a']))
            elif obj['type'] == 'toStr':
                return Expr.from_dict(obj['a']).to_str()
            elif obj['type'] == 'measureTextX':
                return Expr.measure_text_x(Expr.from_dict(obj['text']), Expr.from_dict(obj['fontSize']))
            elif obj['type'] == 'measureTextY':
                return Expr.measure_text_y(Expr.from_dict(obj['text']), Expr.from_dict(obj['fontSize']))
            elif obj['type'] == 'inf':
                return Expr(float('inf'))
            else:
                assert False, f'Unknown type: {obj["type"]}'
        elif isinstance(obj, (int, float, str)):
            return Expr(obj)
        else:
            assert False, f'Unknown obj type: {obj}'

    @staticmethod
    def to_dict(expr: Expr | int | float | str):
        if isinstance(expr, Expr):
            vec = _lib.simplexp_serialize_expr(expr._inner)
            deserialized = json.loads(bytes(_ffi.buffer(vec.ptr, vec.len)))
            _lib.simplexp_free_str(vec)
            return deserialized
        else:
            return expr

    @staticmethod
    def to_ops_dict(expr: Expr | int | float | str):
        if isinstance(expr, Expr):
            vec = _lib.simplexp_serialize_expr_ops(expr._inner)
            deserialized = json.loads(bytes(_ffi.buffer(vec.ptr, vec.len)))
            _lib.simplexp_free_str(vec)
            return deserialized
        else:
            return expr


def var(name: str):
    return Expr(_lib.simplexp_new_var(bytes(name, 'utf-8')))
