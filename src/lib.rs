extern crate proc_macro;
use litrs::Literal;
use proc_macro::TokenStream;

#[proc_macro]
pub fn hash(item: TokenStream) -> TokenStream {
    let tok = item.into_iter().next().unwrap();
    match Literal::try_from(tok) {
        Err(e) => e.to_compile_error(),

        Ok(Literal::String(s)) => format!(
            "0x{:0>16x} as u64",
            xxh3::hash64_with_seed(s.value().as_bytes(), 0)
        )
        .parse()
        .unwrap(),
        _ => {
            panic!("expected string literal")
        }
    }
}
