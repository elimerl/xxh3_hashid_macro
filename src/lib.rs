//! ```rust
//! use xxh3_hashid_macro::hash;
//! let id = hash!("example_id");
//! println!("0x{:0>16x}", id); // 0xf826a9bb47f7ff34
//! ```
extern crate proc_macro;
use litrs::Literal;
use proc_macro::TokenStream;

/// Accepts a string literal and returns its XXH3 64-bit hash as a u64. Uses a non-configurable seed of 0.
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
