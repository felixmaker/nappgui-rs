use proc_macro::TokenStream;

mod dbind;
mod resource;
mod ui;

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

#[proc_macro]
pub fn ui(input: TokenStream) -> TokenStream {
    let input2 = proc_macro2::TokenStream::from(input);
    let output2 = ui::process_ui_macro(input2);
    TokenStream::from(output2)
}
