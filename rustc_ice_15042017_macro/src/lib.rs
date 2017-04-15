#![feature(proc_macro)]

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn rustc_ice(_: TokenStream, x: TokenStream) -> TokenStream {
    x
}
