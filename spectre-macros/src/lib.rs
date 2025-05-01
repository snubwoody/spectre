use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

/// Use a single browser between tests.
#[proc_macro_attribute]
pub fn test(_: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = syn::parse_macro_input!(input as ItemFn);

    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let attrs = &input_fn.attrs;
    let block = &input_fn.block;

    let ident = &sig.ident;
    let output = &sig.output;

    if sig.asyncness.is_none() {
        return syn::Error::new_spanned(
            sig.fn_token,
            "the `async` keyword is missing from the function declaration, functions marked with `spectre::test` must be async",
        )
        .to_compile_error()
        .into();
    }

    let _ = dotenv::dotenv();
    let var = std::env::var("SPECTRE_TEST_PORT").expect("please set `SPECTRE_TEST_PORT`");
    let port: u16 = var.parse().unwrap();

    quote! {
        #[tokio::test]
        #(#attrs)*
        #vis async fn #ident() #output {
            let is_running = spectre::Browser::is_running(#port).await;

            let mut browser = if is_running{
                spectre::Browser::connect(#port).await.unwrap()
            }else{
                spectre::Browser::start_on(#port).await.unwrap()
            };

            browser.kill_on_drop(false);

            // compile_error!("Failed to compile");
            let mut page = browser.new_page().await.unwrap();
            #block
        }
    }
    .into()
}
