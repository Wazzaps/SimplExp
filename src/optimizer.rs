use crate::expressions::{ExprOp, ExprPart};
use std::sync::Arc;

pub fn optimize(expr: Arc<ExprPart>) -> Arc<ExprPart> {
    match &*expr {
        ExprPart::Operation(op) => match op {
            ExprOp::Var { .. } => expr,
            ExprOp::Add { a, b } => match (&**a, &**b) {
                // Optimization: x + -x ≡ 0
                (a, ExprPart::Operation(ExprOp::Neg { a: b })) if a.eq(b) => {
                    Arc::new(ExprPart::IntLiteral(0))
                }

                // Optimization: x + 0 ≡ x
                (_, ExprPart::IntLiteral(0)) => a.clone(),
                (ExprPart::IntLiteral(0), _) => b.clone(),
                (_, ExprPart::FloatLiteral(v)) if v.eq(&0.0) => a.clone(),
                (ExprPart::FloatLiteral(v), _) if v.eq(&0.0) => b.clone(),

                // Optimization: a + b ≡ (a+b)
                (ExprPart::IntLiteral(a), ExprPart::IntLiteral(b)) => {
                    Arc::new(ExprPart::IntLiteral(a + b))
                }
                (ExprPart::FloatLiteral(a), ExprPart::IntLiteral(b)) => {
                    Arc::new(ExprPart::FloatLiteral(a + *b as f64))
                }
                (ExprPart::IntLiteral(a), ExprPart::FloatLiteral(b)) => {
                    Arc::new(ExprPart::FloatLiteral(*a as f64 + b))
                }
                (ExprPart::FloatLiteral(a), ExprPart::FloatLiteral(b)) => {
                    Arc::new(ExprPart::FloatLiteral(a + b))
                }

                // Optimization: (x + a) + b ≡ x + (a+b)
                (
                    ExprPart::Operation(ExprOp::Add { a: left, b: right }),
                    ExprPart::IntLiteral(b),
                ) => match &**right {
                    ExprPart::IntLiteral(v) => {
                        optimize(Arc::new(ExprPart::Operation(ExprOp::Add {
                            a: left.clone(),
                            b: Arc::new(ExprPart::IntLiteral(v + b)),
                        })))
                    }
                    ExprPart::FloatLiteral(v) => {
                        optimize(Arc::new(ExprPart::Operation(ExprOp::Add {
                            a: left.clone(),
                            b: Arc::new(ExprPart::FloatLiteral(v + *b as f64)),
                        })))
                    }
                    _ => expr,
                },
                (
                    ExprPart::Operation(ExprOp::Add { a: left, b: right }),
                    ExprPart::FloatLiteral(b),
                ) => match &**right {
                    ExprPart::IntLiteral(v) => {
                        optimize(Arc::new(ExprPart::Operation(ExprOp::Add {
                            a: left.clone(),
                            b: Arc::new(ExprPart::FloatLiteral(*v as f64 + b)),
                        })))
                    }
                    ExprPart::FloatLiteral(v) => {
                        optimize(Arc::new(ExprPart::Operation(ExprOp::Add {
                            a: left.clone(),
                            b: Arc::new(ExprPart::FloatLiteral(v + b)),
                        })))
                    }
                    _ => expr,
                },

                // Optimization: a + x ≡ x + a
                // This activates the rest of the optimization rules above
                (ExprPart::IntLiteral(_), ExprPart::Operation(_)) => {
                    optimize(Arc::new(ExprPart::Operation(ExprOp::Add {
                        a: b.clone(),
                        b: a.clone(),
                    })))
                }

                // Optimization: x + inf ≡ inf
                (_, ExprPart::Operation(ExprOp::Inf)) => Arc::new(ExprPart::Operation(ExprOp::Inf)),
                (ExprPart::Operation(ExprOp::Inf), _) => Arc::new(ExprPart::Operation(ExprOp::Inf)),

                (_, _) => expr,
            },
            ExprOp::Mul { a, b } => match (&**a, &**b) {
                // Optimization: x * 1 ≡ x
                (_, ExprPart::IntLiteral(1)) => a.clone(),
                (_, ExprPart::FloatLiteral(v)) if v.eq(&1.0) => a.clone(),

                (_, _) => expr,
            },
            ExprOp::Div { a, b } => match (&**a, &**b) {
                // Optimization: x / 1 ≡ x
                (_, ExprPart::IntLiteral(1)) => a.clone(),
                (_, ExprPart::FloatLiteral(v)) if v.eq(&1.0) => a.clone(),

                // Optimization: 0 / x ≡ 0
                (ExprPart::IntLiteral(0), _) => Arc::new(ExprPart::IntLiteral(0)),
                (ExprPart::FloatLiteral(v), _) if v.eq(&0.0) => Arc::new(ExprPart::IntLiteral(0)),

                // Optimization: (x + x) / 2 ≡ x
                (ExprPart::Operation(ExprOp::Add { a, b }), ExprPart::IntLiteral(2)) if a.eq(b) => {
                    a.clone()
                }

                (_, _) => expr,
            },
            ExprOp::Fdiv { a, b } => match (&**a, &**b) {
                // XXX: This is technically incorrect
                // Optimization: x / 1 ≡ x
                (_, ExprPart::IntLiteral(1)) => a.clone(),
                (_, ExprPart::FloatLiteral(v)) if v.eq(&1.0) => a.clone(),

                // Optimization: 0 // x ≡ 0
                (ExprPart::IntLiteral(0), _) => Arc::new(ExprPart::IntLiteral(0)),
                (ExprPart::FloatLiteral(v), _) if v.eq(&0.0) => Arc::new(ExprPart::IntLiteral(0)),

                // Optimization: (x + x) // 2 ≡ x // 1
                (ExprPart::Operation(ExprOp::Add { a, b }), ExprPart::IntLiteral(2)) if a.eq(b) => {
                    optimize(Arc::new(ExprPart::Operation(ExprOp::Fdiv {
                        a: a.clone(),
                        b: Arc::new(ExprPart::IntLiteral(1)),
                    })))
                }

                (_, _) => expr,
            },
            // ExprOp::Mod { a, b } => unimplemented!(),
            // ExprOp::Pow { a, b } => unimplemented!(),
            // ExprOp::Eq { a, b } => unimplemented!(),
            // ExprOp::Neq { a, b } => unimplemented!(),
            // ExprOp::Lt { a, b } => unimplemented!(),
            // ExprOp::Lte { a, b } => unimplemented!(),
            // ExprOp::Gt { a, b } => unimplemented!(),
            // ExprOp::Gte { a, b } => unimplemented!(),
            // ExprOp::BAnd { a, b } => unimplemented!(),
            // ExprOp::BOr { a, b } => unimplemented!(),
            // ExprOp::Not { a } => unimplemented!(),
            ExprOp::Neg { a } => match &**a {
                // Optimization: -a ≡ -a
                ExprPart::IntLiteral(v) => Arc::new(ExprPart::IntLiteral(-*v)),
                ExprPart::FloatLiteral(v) => Arc::new(ExprPart::FloatLiteral(-*v)),

                _ => expr,
            },
            // ExprOp::BInvert { a } => unimplemented!(),
            ExprOp::Min { a, b } => match (&**a, &**b) {
                // Optimization: min(x, inf) ≡ inf
                (_, ExprPart::Operation(ExprOp::Inf)) => a.clone(),
                (ExprPart::Operation(ExprOp::Inf), _) => b.clone(),

                // Optimization: min(x, y) ≡ min(x, y)
                (ExprPart::IntLiteral(a), ExprPart::IntLiteral(b)) => {
                    Arc::new(ExprPart::IntLiteral((*a).min(*b)))
                }
                (ExprPart::FloatLiteral(a), ExprPart::IntLiteral(b)) => {
                    Arc::new(ExprPart::FloatLiteral((*a).min(*b as f64)))
                }
                (ExprPart::IntLiteral(a), ExprPart::FloatLiteral(b)) => {
                    Arc::new(ExprPart::FloatLiteral((*a as f64).min(*b)))
                }
                (ExprPart::FloatLiteral(a), ExprPart::FloatLiteral(b)) => {
                    Arc::new(ExprPart::FloatLiteral((*a).min(*b)))
                }

                (_, _) => expr,
            },
            ExprOp::Max {
                a: a_expr,
                b: b_expr,
            } => match (&**a_expr, &**b_expr) {
                // Optimization: max(x, inf) ≡ x
                (_, ExprPart::Operation(ExprOp::Inf)) => Arc::new(ExprPart::Operation(ExprOp::Inf)),
                (ExprPart::Operation(ExprOp::Inf), _) => Arc::new(ExprPart::Operation(ExprOp::Inf)),

                // Optimization: max(x, y) ≡ max(x, y)
                (ExprPart::IntLiteral(a), ExprPart::IntLiteral(b)) => {
                    Arc::new(ExprPart::IntLiteral((*a).max(*b)))
                }
                (ExprPart::FloatLiteral(a), ExprPart::IntLiteral(b)) => {
                    Arc::new(ExprPart::FloatLiteral((*a).max(*b as f64)))
                }
                (ExprPart::IntLiteral(a), ExprPart::FloatLiteral(b)) => {
                    Arc::new(ExprPart::FloatLiteral((*a as f64).max(*b)))
                }
                (ExprPart::FloatLiteral(a), ExprPart::FloatLiteral(b)) => {
                    Arc::new(ExprPart::FloatLiteral((*a).max(*b)))
                }

                // Optimization: max(-c, measureText(...)) ≡ measureText(...)
                // MeasureText is non-negative
                (op, ExprPart::IntLiteral(c))
                    if *c <= 0
                        && matches!(op, ExprPart::Operation(ExprOp::MeasureTextX { .. })) =>
                {
                    a_expr.clone()
                }
                (ExprPart::IntLiteral(c), op)
                    if *c <= 0
                        && matches!(op, ExprPart::Operation(ExprOp::MeasureTextX { .. })) =>
                {
                    b_expr.clone()
                }
                (op, ExprPart::FloatLiteral(c))
                    if *c <= 0.0
                        && matches!(op, ExprPart::Operation(ExprOp::MeasureTextX { .. })) =>
                {
                    a_expr.clone()
                }
                (ExprPart::FloatLiteral(c), op)
                    if *c <= 0.0
                        && matches!(op, ExprPart::Operation(ExprOp::MeasureTextX { .. })) =>
                {
                    b_expr.clone()
                }
                (op, ExprPart::IntLiteral(c))
                    if *c <= 0
                        && matches!(op, ExprPart::Operation(ExprOp::MeasureTextY { .. })) =>
                {
                    a_expr.clone()
                }
                (ExprPart::IntLiteral(c), op)
                    if *c <= 0
                        && matches!(op, ExprPart::Operation(ExprOp::MeasureTextY { .. })) =>
                {
                    b_expr.clone()
                }
                (op, ExprPart::FloatLiteral(c))
                    if *c <= 0.0
                        && matches!(op, ExprPart::Operation(ExprOp::MeasureTextY { .. })) =>
                {
                    a_expr.clone()
                }
                (ExprPart::FloatLiteral(c), op)
                    if *c <= 0.0
                        && matches!(op, ExprPart::Operation(ExprOp::MeasureTextY { .. })) =>
                {
                    b_expr.clone()
                }

                (_, _) => expr,
            },
            // ExprOp::Abs { a } => unimplemented!(),
            // ExprOp::Inf => expr,
            // ExprOp::ToStr { a } => unimplemented!(),
            // ExprOp::MeasureTextX { text, font_size } => unimplemented!(),
            // ExprOp::MeasureTextY { text, font_size } => unimplemented!(),
            _ => expr,
        },
        _ => expr,
    }
}
