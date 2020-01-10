#![feature(box_syntax, box_patterns)]
extern crate proc_macro;
extern crate proc_quote;
extern crate risp_ast;

use proc_macro::{Delimiter, Literal, Punct, TokenStream, TokenTree};
use proc_macro_hack::proc_macro_hack;
use proc_quote::quote;
use risp_ast::{Expression, Operation};

#[proc_macro_hack]
pub fn risp(input: TokenStream) -> TokenStream {
    let tree = input.into_iter().next().unwrap();
    let expression = parse_expression(&tree);
    let interpreted = interpret_expression(expression);

    TokenStream::from(quote! {
        #interpreted
    })
}

fn parse_expression(tree: &TokenTree) -> Expression {
    match tree {
        TokenTree::Group(_) => parse_apply(tree),
        TokenTree::Literal(literal) => parse_literal(literal),
        _ => panic!("Could not parse expression"),
    }
}

fn parse_apply(tree: &TokenTree) -> Expression {
    match tree {
        TokenTree::Group(group) if group.delimiter() == Delimiter::Parenthesis => {
            let token_trees: Vec<TokenTree> = group.stream().into_iter().collect();
            match &token_trees.as_slice() {
                &[TokenTree::Punct(punct), first_arg, second_arg] => Expression::Apply {
                    operation: parse_operation(punct),
                    first_arg: Box::new(parse_expression(first_arg)),
                    second_arg: Box::new(parse_expression(second_arg)),
                },
                _ => panic!("Could not parse apply"),
            }
        }
        _ => panic!("Expected a parse apply tree to start with a group and paranthesis delimiter"),
    }
}

fn parse_operation(punct: &Punct) -> Operation {
    match punct.as_char() {
        '+' => Operation::Add,
        '-' => Operation::Subtract,
        '*' => Operation::Multiply,
        '/' => Operation::Divide,
        _ => panic!("Unsupported operation"),
    }
}

fn parse_literal(literal: &Literal) -> Expression {
    match literal.to_string().parse::<i64>() {
        Ok(int) => Expression::Integer { value: int },
        _ => panic!("Invalid literal. I only like integers"),
    }
}

fn interpret_apply(operation: Operation, first_arg: Expression, second_arg: Expression) -> i64 {
    let evaled_first_arg = interpret_expression(first_arg);
    let evaled_second_arg = interpret_expression(second_arg);
    match operation {
        Operation::Add => evaled_first_arg + evaled_second_arg,
        Operation::Subtract => evaled_first_arg - evaled_second_arg,
        Operation::Multiply => evaled_first_arg * evaled_second_arg,
        Operation::Divide => evaled_first_arg / evaled_second_arg,
    }
}

fn interpret_expression(expression: Expression) -> i64 {
    match expression {
        Expression::Integer { value: int } => int,
        Expression::Apply {
            operation,
            box first_arg,
            box second_arg,
        } => interpret_apply(operation, first_arg, second_arg),
    }
}
