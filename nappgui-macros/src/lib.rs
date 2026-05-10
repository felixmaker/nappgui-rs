use proc_macro::TokenStream;

mod dbind;
mod resource;

#[proc_macro]
pub fn include_resource(input: TokenStream) -> TokenStream {
    resource::include_resource(input)
}

#[proc_macro_derive(DataBind)]
pub fn data_bind(input: TokenStream) -> TokenStream {
    let input2 = proc_macro2::TokenStream::from(input);
    let output2 = dbind::impl_data_bind(input2);
    proc_macro::TokenStream::from(output2)
}
