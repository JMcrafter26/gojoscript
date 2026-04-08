use crate::ast::*;

struct Prng {
    state: u32,
}

impl Prng {
    fn new(seed: u32) -> Self {
        Self {
            state: if seed == 0 { 1 } else { seed },
        }
    }

    fn next_u32(&mut self) -> u32 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 17;
        self.state ^= self.state << 5;
        self.state
    }
}

pub fn visit_project(project: &mut Project) {
    let mut prng = Prng::new(0x27182818); // fixed seed for reproducibility

    visit_sprite(&mut project.stage, &mut prng);
    for sprite in project.sprites.values_mut() {
        visit_sprite(sprite, &mut prng);
    }
}

fn visit_sprite(sprite: &mut Sprite, prng: &mut Prng) {
    for proc in sprite.proc_definitions.values_mut() {
        visit_stmts(proc, prng);
    }
    for func in sprite.func_definitions.values_mut() {
        visit_stmts(func, prng);
    }
    for event in &mut sprite.events {
        visit_stmts(&mut event.body, prng);
    }
}

fn visit_stmts(stmts: &mut [Stmt], prng: &mut Prng) {
    for stmt in stmts {
        visit_stmt(stmt, prng);
    }
}

fn visit_stmt(stmt: &mut Stmt, prng: &mut Prng) {
    match stmt {
        Stmt::Repeat { times, body } => {
            visit_expr(times, prng);
            visit_stmts(body, prng);
        }
        Stmt::Forever { body, .. } => visit_stmts(body, prng),
        Stmt::Branch {
            cond,
            if_body,
            else_body,
        } => {
            visit_expr(cond, prng);
            visit_stmts(if_body, prng);
            visit_stmts(else_body, prng);
        }
        Stmt::Until { cond, body } => {
            visit_expr(cond, prng);
            visit_stmts(body, prng);
        }
        Stmt::SetVar { value, .. } => visit_expr(value, prng),
        Stmt::ChangeVar { value, .. } => visit_expr(value, prng),
        Stmt::Show(_) | Stmt::Hide(_) => {}
        Stmt::AddToList { value, .. } => visit_expr(value, prng),
        Stmt::DeleteList(_) => {}
        Stmt::DeleteListIndex { index, .. } => visit_expr(index, prng),
        Stmt::InsertAtList { index, value, .. } => {
            visit_expr(index, prng);
            visit_expr(value, prng);
        }
        Stmt::SetListIndex { index, value, .. } => {
            visit_expr(index, prng);
            visit_expr(value, prng);
        }
        Stmt::Block { args, kwargs, .. } => {
            for arg in args {
                visit_expr(arg, prng);
            }
            for arg in kwargs.values_mut() {
                visit_expr(&mut arg.1, prng);
            }
        }
        Stmt::ProcCall { args, kwargs, .. } => {
            for arg in args {
                visit_expr(arg, prng);
            }
            for arg in kwargs.values_mut() {
                visit_expr(&mut arg.1, prng);
            }
        }
        Stmt::FuncCall { args, kwargs, .. } => {
            for arg in args {
                visit_expr(arg, prng);
            }
            for arg in kwargs.values_mut() {
                visit_expr(&mut arg.1, prng);
            }
        }
        Stmt::Return { .. } => {}
    }
}

fn visit_expr(expr: &mut Expr, prng: &mut Prng) {
    if let Expr::Value { value: Value::Number(n), span } = expr {
        if n.is_nan() || n.is_infinite() {
            return;
        }

        let number = *n;
        let r = (prng.next_u32() % 10000) as f64 + 1.0; 

        let (op, lhs_val, rhs_val) = if number == 0.0 {
            match prng.next_u32() % 4 {
                0 => (crate::blocks::BinOp::Sub, r, r),
                1 => (crate::blocks::BinOp::Div, 0.0, r),
                2 => (crate::blocks::BinOp::Mul, 0.0, r),
                _ => (crate::blocks::BinOp::Mul, r, 0.0),
            }
        } else {
            match prng.next_u32() % 6 {
                // R / (R / N)
                0 => (crate::blocks::BinOp::Div, r, r / number),
                // R - (R - N)
                1 => (crate::blocks::BinOp::Sub, r, r - number),
                // (N - R) + R
                2 => (crate::blocks::BinOp::Add, number - r, r),
                // (N * R) / R
                3 => (crate::blocks::BinOp::Div, number * r, r),
                // (N + R) - R
                4 => (crate::blocks::BinOp::Sub, number + r, r),
                // R + (N - R)
                _ => (crate::blocks::BinOp::Add, r, number - r),
            }
        };

        // Replace the current expression!
        *expr = Expr::BinOp {
            op,
            span: span.clone(),
            lhs: Box::new(Expr::Value {
                value: Value::Number(lhs_val),
                span: span.clone(),
            }),
            rhs: Box::new(Expr::Value {
                value: Value::Number(rhs_val),
                span: span.clone(),
            }),
        };
        return;
    }

    match expr {
        Expr::Value { .. } => {}
        Expr::Name(_) => {}
        Expr::Dot { lhs, .. } => visit_expr(lhs, prng),
        Expr::Arg(_) => {}
        Expr::Repr { args, .. } => {
            for arg in args {
                visit_expr(arg, prng);
            }
        }
        Expr::FuncCall {
            args: _args,
            kwargs: _kwargs,
            ..
        } => {
            // function calls should be folded or parsed already
        }
        Expr::UnOp { opr, .. } => {
            visit_expr(opr, prng);
        }
        Expr::BinOp { lhs, rhs, .. } => {
            visit_expr(lhs, prng);
            visit_expr(rhs, prng);
        }
        Expr::StructLiteral { fields, .. } => {
            for field in fields {
                visit_expr(&mut field.value, prng);
            }
        }
        Expr::Property { object, .. } => {
            visit_expr(object, prng);
        }
    }
}
