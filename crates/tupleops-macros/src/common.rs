use proc_macro::TokenStream;
use syn::parse_macro_input::parse;
use syn::{Expr, ExprLit, ExprRange, Lit, RangeLimits};

fn get_usize(expr: &Option<Box<Expr>>) -> Result<usize, ()> {
    let expr = match expr {
        Some(expr) => expr.as_ref(),
        None => return Err(()),
    };
    let lit = match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Int(lit), ..
        }) => lit,
        _ => return Err(()),
    };
    match lit.base10_parse() {
        Ok(value) => Ok(value),
        Err(_) => Err(()),
    }
}

fn parse_range(input: TokenStream) -> Result<(usize, usize), ()> {
    let range: ExprRange = match parse(input) {
        Ok(range) => range,
        Err(_) => return Err(()),
    };
    if !matches!(range.limits, RangeLimits::Closed(_)) {
        return Err(());
    }
    let from = match get_usize(&range.from) {
        Ok(from) => from,
        Err(_) => return Err(()),
    };
    let to = match get_usize(&range.to) {
        Ok(to) => to,
        Err(_) => return Err(()),
    };
    Ok((from, to))
}

pub(crate) fn generate(
    input: TokenStream,
    gen: fn(&mut String, usize, usize) -> Result<(), std::fmt::Error>,
) -> TokenStream {
    if let Ok((from, to)) = parse_range(input) {
        let mut code = String::new();
        if gen(&mut code, from, to).is_ok() {
            return code.parse().unwrap();
        }
    }
    r#"::core::compile_error("Expected closed range of usizes")"#
        .parse()
        .unwrap()
}
