use pom::parser::*;
use std::fs;
use itertools::Itertools;


#[derive(Debug)]
enum Expr<T> {
    Add(Box<Expr<T>>, Box<Expr<T>>),
    Mul(Box<Expr<T>>, Box<Expr<T>>),
    Constant(T),
}

impl<T> Expr<T> {
    fn create_add((left, right): (Expr<T>, Expr<T>)) -> Expr<T> {
        Self::Add(Box::new(left), Box::new(right))
    }

    fn create_mul((left, right): (Expr<T>, Expr<T>)) -> Expr<T> {
        Self::Mul(Box::new(left), Box::new(right))
    }
}

fn integer<'a>() -> Parser<'a, u8, Expr<u64>> {
    one_of(b"0123456789").repeat(1..)
        .convert(String::from_utf8)
        .convert(|s| s.parse::<u64>())
        .map(|val| Expr::Constant(val))
}

fn space<'a>() -> Parser<'a, u8, ()> {
    sym(b' ').repeat(0..).discard()
}

fn value<'a>() -> Parser<'a, u8, Expr<u64>> {
    integer() | parentheses()
}

fn parentheses<'a>() -> Parser<'a, u8, Expr<u64>> {
    sym(b'(') * space() * call(expression) - space() - sym(b')')
}

fn expression<'a>() -> Parser<'a, u8, Expr<u64>> {
    let right_operand = space() * one_of(b"+*") - space() + value();
    let expr = value() + right_operand.repeat(0..);
    expr.map(|(left, right)|
        right.into_iter().fold(left, |curr, (op, r)|
            match op {
                b'+' => Expr::create_add((curr, r)),
                b'*' => Expr::create_mul((curr, r)),
                _ => panic!("Invalid operator")
            },
        ))
}

fn eval_expression(expr: &Expr<u64>) -> u64 {
    match expr {
        Expr::Add(left, right) =>
            eval_expression(left) + eval_expression(right),

        Expr::Mul(left, right) =>
            eval_expression(left) * eval_expression(right),

        Expr::Constant(val) => *val
    }
}

fn main() {
    let file = fs::read_to_string("input").expect("Could not open file");
    let expressions : u64 = file.lines()
        .map(|line| expression().parse(line.as_bytes()).unwrap())
        .map(|e| eval_expression(&e))
        .sum();

    println!("Sum of all expressions is {}", expressions);
}