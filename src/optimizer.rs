use crate::{ExprOp, ExprPart};

pub fn optimize(expr: &ExprPart) -> Box<ExprPart> {
    Box::new(match expr {
        ExprPart::Operation(op) => match op {
            ExprOp::Var { .. } => expr.clone(),
            ExprOp::Add { a, b } => match (&**a, &**b) {
                // Optimization: x - x ≡ 0
                (a, ExprPart::Operation(ExprOp::Neg { a: b })) if a.eq(b) => {
                    ExprPart::IntLiteral(0)
                }
                (_, _) => expr.clone(),
            },
            ExprOp::Mul { a, b } => unimplemented!(),
            ExprOp::Div { a, b } => unimplemented!(),
            ExprOp::Fdiv { a, b } => unimplemented!(),
            ExprOp::Mod { a, b } => unimplemented!(),
            ExprOp::Pow { a, b } => unimplemented!(),
            ExprOp::Eq { a, b } => unimplemented!(),
            ExprOp::Neq { a, b } => unimplemented!(),
            ExprOp::Lt { a, b } => unimplemented!(),
            ExprOp::Lte { a, b } => unimplemented!(),
            ExprOp::Gt { a, b } => unimplemented!(),
            ExprOp::Gte { a, b } => unimplemented!(),
            ExprOp::BAnd { a, b } => unimplemented!(),
            ExprOp::BOr { a, b } => unimplemented!(),
            ExprOp::Not { a } => unimplemented!(),
            ExprOp::Neg { a } => unimplemented!(),
            ExprOp::BInvert { a } => unimplemented!(),
            ExprOp::Min { a, b } => unimplemented!(),
            ExprOp::Max { a, b } => unimplemented!(),
            ExprOp::Abs { a } => unimplemented!(),
            ExprOp::Inf => expr.clone(),
            ExprOp::ToStr { a } => unimplemented!(),
            ExprOp::MeasureTextX { text, font_size } => unimplemented!(),
            ExprOp::MeasureTextY { text, font_size } => unimplemented!(),
        },
        _ => expr.clone(),
    })
}
