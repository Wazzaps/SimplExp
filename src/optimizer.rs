use crate::{ExprOp, ExprPart};
use std::sync::Arc;

fn optimize_single(expr: Arc<ExprPart>) -> Arc<ExprPart> {
    // println!("optimize_single: {:?}", expr);
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
                (_, _) => expr,
            },
            // ExprOp::Mul { a, b } => unimplemented!(),
            // ExprOp::Div { a, b } => unimplemented!(),
            // ExprOp::Fdiv { a, b } => unimplemented!(),
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
            // ExprOp::Neg { a } => unimplemented!(),
            // ExprOp::BInvert { a } => unimplemented!(),
            // ExprOp::Min { a, b } => unimplemented!(),
            // ExprOp::Max { a, b } => unimplemented!(),
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

pub fn optimize(expr: Arc<ExprPart>) -> Arc<ExprPart> {
    // println!("optimize: {:?}", expr);
    match &*expr {
        ExprPart::Operation(op) => match op {
            ExprOp::Var { .. } => expr,
            ExprOp::Add { a, b } => optimize_single(Arc::new(ExprPart::Operation(ExprOp::Add {
                a: optimize(a.clone()),
                b: optimize(b.clone()),
            }))),
            ExprOp::Mul { a, b } => optimize_single(Arc::new(ExprPart::Operation(ExprOp::Mul {
                a: optimize(a.clone()),
                b: optimize(b.clone()),
            }))),
            ExprOp::Div { a, b } => optimize_single(Arc::new(ExprPart::Operation(ExprOp::Div {
                a: optimize(a.clone()),
                b: optimize(b.clone()),
            }))),
            ExprOp::Fdiv { a, b } => optimize_single(Arc::new(ExprPart::Operation(ExprOp::Fdiv {
                a: optimize(a.clone()),
                b: optimize(b.clone()),
            }))),
            ExprOp::Mod { a, b } => optimize_single(Arc::new(ExprPart::Operation(ExprOp::Mod {
                a: optimize(a.clone()),
                b: optimize(b.clone()),
            }))),
            ExprOp::Pow { a, b } => optimize_single(Arc::new(ExprPart::Operation(ExprOp::Pow {
                a: optimize(a.clone()),
                b: optimize(b.clone()),
            }))),
            ExprOp::Eq { a, b } => optimize_single(Arc::new(ExprPart::Operation(ExprOp::Eq {
                a: optimize(a.clone()),
                b: optimize(b.clone()),
            }))),
            ExprOp::Neq { a, b } => optimize_single(Arc::new(ExprPart::Operation(ExprOp::Neq {
                a: optimize(a.clone()),
                b: optimize(b.clone()),
            }))),
            ExprOp::Lt { a, b } => optimize_single(Arc::new(ExprPart::Operation(ExprOp::Lt {
                a: optimize(a.clone()),
                b: optimize(b.clone()),
            }))),
            ExprOp::Lte { a, b } => optimize_single(Arc::new(ExprPart::Operation(ExprOp::Lte {
                a: optimize(a.clone()),
                b: optimize(b.clone()),
            }))),
            ExprOp::Gt { a, b } => optimize_single(Arc::new(ExprPart::Operation(ExprOp::Gt {
                a: optimize(a.clone()),
                b: optimize(b.clone()),
            }))),
            ExprOp::Gte { a, b } => optimize_single(Arc::new(ExprPart::Operation(ExprOp::Gte {
                a: optimize(a.clone()),
                b: optimize(b.clone()),
            }))),
            ExprOp::BAnd { a, b } => optimize_single(Arc::new(ExprPart::Operation(ExprOp::BAnd {
                a: optimize(a.clone()),
                b: optimize(b.clone()),
            }))),
            ExprOp::BOr { a, b } => optimize_single(Arc::new(ExprPart::Operation(ExprOp::BOr {
                a: optimize(a.clone()),
                b: optimize(b.clone()),
            }))),
            ExprOp::Not { a } => optimize_single(Arc::new(ExprPart::Operation(ExprOp::Not {
                a: optimize(a.clone()),
            }))),
            ExprOp::Neg { a } => optimize_single(Arc::new(ExprPart::Operation(ExprOp::Neg {
                a: optimize(a.clone()),
            }))),
            ExprOp::BInvert { a } => {
                optimize_single(Arc::new(ExprPart::Operation(ExprOp::BInvert {
                    a: optimize(a.clone()),
                })))
            }
            ExprOp::Min { a, b } => optimize_single(Arc::new(ExprPart::Operation(ExprOp::Min {
                a: optimize(a.clone()),
                b: optimize(b.clone()),
            }))),
            ExprOp::Max { a, b } => optimize_single(Arc::new(ExprPart::Operation(ExprOp::Max {
                a: optimize(a.clone()),
                b: optimize(b.clone()),
            }))),
            ExprOp::Abs { a } => optimize_single(Arc::new(ExprPart::Operation(ExprOp::Abs {
                a: optimize(a.clone()),
            }))),
            ExprOp::Inf => expr,
            ExprOp::ToStr { a } => optimize_single(Arc::new(ExprPart::Operation(ExprOp::ToStr {
                a: optimize(a.clone()),
            }))),
            ExprOp::MeasureTextX { text, font_size } => {
                optimize_single(Arc::new(ExprPart::Operation(ExprOp::MeasureTextX {
                    text: optimize(text.clone()),
                    font_size: optimize(font_size.clone()),
                })))
            }
            ExprOp::MeasureTextY { text, font_size } => {
                optimize_single(Arc::new(ExprPart::Operation(ExprOp::MeasureTextY {
                    text: optimize(text.clone()),
                    font_size: optimize(font_size.clone()),
                })))
            }
        },
        _ => expr,
    }
}
