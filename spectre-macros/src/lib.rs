use std::cell::OnceCell;
use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;
use spectre_core::Browser;

// static BROWSER: OnceCell<Browser> = OnceCell::new();

// async fn get_or_init_browser(){
// 	let k = BROWSER.get_or_init(||async{
// 		Browser::launch().await.unwrap()
// 	});
// }

/// Create a page for every test using one browser
/// 
/// ```ignore
/// #[spectre::test]
/// async fn test(page: Page){
/// 
/// }
/// ```
#[proc_macro_attribute]
pub fn test(_: TokenStream,input: TokenStream) -> TokenStream{
	let input_fn = syn::parse_macro_input!(input as ItemFn);
	
	let vis = &input_fn.vis;
	let sig = &input_fn.sig;
	let attrs = &input_fn.attrs;
	let block = &input_fn.block;
	let ident = &sig.ident;
	
	dbg!(vis);
	dbg!(sig);
	dbg!(attrs);
	dbg!(block);

	if !sig.asyncness.is_some(){
		return syn::Error::new_spanned(
			&sig.fn_token, 
			"the `async` keyword is missing from the function declaration"
		)
		.to_compile_error()
		.into()
	}
	
	quote! {
		#[tokio::test]
		#(#attrs)*
		#vis async fn #ident(){
			let mut browser = spectre::Browser::launch().await.unwrap();
			#block
		}
	}.into()
}