use proc_macro_hack::proc_macro_hack;

pub enum Expression {
    FunctionDefinition {
        name: String,
        description: String,
        params: Vec<String>,
        body: Box<Expression>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
    Ident {
        name: String,
    },
    Integer {
        value: i64,
    },
}


#[proc_macro_hack]
pub use risp_macro::add_one;
