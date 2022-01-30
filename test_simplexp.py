from simplexp import var, Expr


def test_simplify(expr):
    print(f'Original: {expr}    Simplified: {expr.simplify()}')


print('--- PRIMITIVES ---')
print(var('x'))
print(Expr('hello world'))
print(Expr(1))
print(Expr(1.23))

print('--- BASIC EXPRESSIONS ---')
print(var('x') + var('y'))
print(var('x') - 5)
print(var('x') - var('x'))

print('--- SIMPLIFICATION ---')
test_simplify(var('x') - var('x'))
test_simplify(var('x') - var('x') + var('y'))
