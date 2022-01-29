# SimplExp

Fast and crude mathematical expression simplifier for Python and Rust

Used in [BoldUI](https://github.com/Wazzaps/boldui).

## Example

```python
from simplexp import var, Expr

original = var('x') + 10 + 10 + var('x')
expected = 2 * var('x') + 20

assert not Expr.is_identical(original, expected)
assert Expr.is_identical(original.simplify(), expected)
```