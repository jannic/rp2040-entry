extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse, parse_macro_input, spanned::Spanned, AttrStyle, Attribute, Ident, ItemFn
};

#[proc_macro_attribute]
pub fn entry(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut f = parse_macro_input!(input as ItemFn);

    if !args.is_empty() {
        return parse::Error::new(Span::call_site(), "This attribute accepts no arguments")
            .to_compile_error()
            .into();
    }

    f.sig.ident = Ident::new(&format!("__rp2040_entry_{}", f.sig.ident), Span::call_site());
    let tramp_ident = Ident::new(&format!("{}_trampoline", f.sig.ident), Span::call_site());
    let ident = &f.sig.ident;
    let attrs = &f.attrs;

    quote!(
        #[cortex_m_rt::entry]
        #(#attrs)*
        unsafe fn #tramp_ident() -> ! {
            const SIO_BASE: u32 = 0xd0000000;
            const SPINLOCK0: u32 = SIO_BASE + 0x100;
            const SPINLOCK_COUNT: u32 = 32;
            for i in 0..SPINLOCK_COUNT {
                core::ptr::write_volatile((SPINLOCK0 + 4 * i) as *mut u32, 1);
            }
            #ident()
        }

        #f
    )
    .into()
}

