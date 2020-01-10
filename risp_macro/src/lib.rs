#![feature(box_syntax, box_patterns)]
extern crate proc_macro;
extern crate proc_quote;
extern crate risp_ast;

use proc_macro::{Delimiter, Literal, Punct, TokenStream, TokenTree};
use proc_macro_hack::proc_macro_hack;
use proc_quote::quote;
use risp_ast::{Expression, Operation};

fn first_tree_from_stream(input: TokenStream) -> TokenTree {
    for tree in input {
        return tree;
    }

    panic!("No tree found!")
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

fn parse_expression(tree: &TokenTree) -> Expression {
    match tree {
        TokenTree::Literal(literal) => parse_literal(literal),
        TokenTree::Group(_) => parse_apply(tree),
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
                    args: Box::new((parse_expression(first_arg), parse_expression(second_arg))),
                },
                _ => panic!("Could not parse apply"),
            }
        }
        _ => panic!("Expected a parse apply tree to start with a group and paranthesis delimiter"),
    }
}

fn parse_risp_expression(input: TokenStream) -> Expression {
    let tree = first_tree_from_stream(input);
    parse_apply(&tree)
}

fn evaluate_apply(operation: Operation, (arg1, arg2): (Expression, Expression)) -> i64 {
    let evaled_arg1 = evaluate_expression(arg1);
    let evaled_arg2 = evaluate_expression(arg2);
    match operation {
        Operation::Add => evaled_arg1 + evaled_arg2,
        Operation::Subtract => evaled_arg1 - evaled_arg2,
        Operation::Multiply => evaled_arg1 * evaled_arg2,
        Operation::Divide => evaled_arg1 / evaled_arg2,
    }
}

fn evaluate_expression(expression: Expression) -> i64 {
    match expression {
        Expression::Integer { value: int } => int,
        Expression::Apply {
            operation,
            box args,
        } => evaluate_apply(operation, args),
    }
}

#[proc_macro_hack]
pub fn risp(input: TokenStream) -> TokenStream {
    let expression = parse_risp_expression(input);
    println!("{:#?}", expression);
    let evaluated = evaluate_expression(expression);

    TokenStream::from(quote! {
        #evaluated
    })
}
