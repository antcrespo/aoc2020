use std::fs;


struct Constant {
    value: u64,
}

struct Operation {
    left: Box<Expression>,
    right: Box<Expression>,
    operator: char,
    paren: bool,
}

enum Expression {
    Const(Constant),
    Op(Operation),
}

fn evaluate(expression: &Expression) -> u64 {
    match &*expression {
        Expression::Const(constant) => (constant.value),
        Expression::Op(operation) => (evaluate_operation(&*operation)),
    }
}

fn evaluate_gold(expression: Expression) -> u64 {
    match expression {
        Expression::Const(constant) => (constant.value),
        Expression::Op(operation) => (evaluate_operation_gold(operation)),
    }
}

fn evaluate_operation(operation: &Operation) -> u64 {
    match operation.operator {
        '+' => evaluate(&*operation.left) + evaluate(&*operation.right),
        '*' => evaluate(&*operation.left) * evaluate(&*operation.right),
        _ => 0,
    }
}

fn evaluate_operation_gold(operation: Operation) -> u64 {
    match operation.operator {
        '+' => {
            match *operation.right {
                //break apart addition to respect order of operations
                //since things are read evaluated left to right, a right side constant is terminal
                Expression::Const(constant) => (evaluate_gold(*operation.left) + constant.value),
                Expression::Op(operation2) => {
                    if operation2.paren {
                        //on parentheses don't rewrite the expression
                        return evaluate_gold(*operation.left) + evaluate_operation_gold(operation2);
                    }
                    // rewrites Expression so that a+(b*c) becomes (a+b)*c (remember there aren't actually any parentheses here)
                    let new_left = Expression::Op(Operation {paren: false, left: operation.left, operator: '+', right: operation2.left});
                    let new_op = Operation {paren: false, left: Box::new(new_left), operator: operation2.operator, right: operation2.right};
                    return evaluate_operation_gold(new_op);
                },
            }
        }
        '*' => evaluate_gold(*operation.left) * evaluate_gold(*operation.right),
        _ => 0,
    }
}

fn main() {

    let input = fs::read_to_string("./src/input/day18.txt")
        .expect("Something went wrong reading the file");
    let lines = input.split("\r\n");
    let mut silver = 0;
    let mut gold = 0;
    for line in lines {
        let expr = create_expression(line.as_bytes(), false).0;
        silver += evaluate(&expr);
        gold += evaluate_gold(expr);
    }
    println!("Silver: {}", silver);
    println!("Gold: {}", gold);

}

fn create_expression(expression: &[u8], paren: bool) -> (Expression, i32) {
    let mut expecting_operator = false;
    let mut i: i32 = (expression.len()-1) as i32;
    let mut current_left: Expression = Expression::Const(Constant {value: 0});
    while i >= 0 {
        let cur = expression[i as usize] as char;
        if cur == ' ' {
            i -= 1;
            continue;
        } else if (cur == '+' || cur == '*') && expecting_operator {
            let (expr, j) = create_expression(&expression[..i as usize], false);
            return (Expression::Op(Operation {paren: paren, left: Box::new(current_left), operator: cur, right: Box::new(expr)}), j);
        } else if !expecting_operator {
            if cur == ')' {
                let (expr, j) = create_expression(&expression[..i as usize], true);
                current_left = expr;
                i = j;
                expecting_operator = true;
            } else {
                let val = cur.to_digit(10).expect("Invalid digit") as u64;
                current_left = Expression::Const(Constant {value: val});
                expecting_operator = true;
                i -= 1;
            }
        }  else if cur == '(' {
            return (current_left, i-1);
        }
    }
    return (current_left, 0);
}
