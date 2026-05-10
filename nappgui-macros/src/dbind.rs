use std::ffi::CString;

use proc_macro2::{TokenStream, Literal};
use quote::{quote};
use syn::{Data, DeriveInput, Fields, Type, TypePath, parse2};

pub fn impl_data_bind(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse2(input).expect("Unable to parse struct");
    let struct_ident = input.ident;
    let struct_literal = Literal::c_string(&CString::new(struct_ident.to_string()).unwrap());

    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named,
            _ => panic!("Named fields only"),
        },
        _ => panic!("Structs only"),
    };

    let field_data_bind = fields.iter().map(|f| {
        let Some(field_ident) = f.ident.as_ref() else {return quote! {}};
        let Type::Path(field_type_path) = &f.ty else {return quote! {}};

        let field_ident_literal = Literal::c_string(&CString::new(field_ident.to_string()).unwrap());
        let Some(field_type_literal) = nappgui_type(field_type_path) else {return quote! {}};
        
        quote! {
            dbind.add_field(
                #field_ident_literal,
                #field_type_literal,
                std::mem::offset_of!(#struct_ident, #field_ident) as _,
                std::mem::size_of::<#field_type_path>() as _,
            );
        }
    });

    quote! {
        impl #struct_ident {
            /// Register the struct to global dbind so that you can bind it to layout.
            pub fn dbind_register() -> Result<(), nappgui::error::NappguiError> {
                let dbind = DBindStruct::new(#struct_literal, size_of::<#struct_ident>() as _);
                #(#field_data_bind)*
                nappgui::core::dbind_register(dbind)
            }
        }
    }
}

fn nappgui_type(path: &TypePath) -> Option<Literal> {
    let last = path.path.segments.last()?;
    let ident = &last.ident;
    match ident {
        _ if ident == "NappguiString"=> Some(Literal::c_string(c"String")),
        _ if ident == "NappguiInt"  || ident == "i32" => Some(Literal::c_string(c"int32_t")),
        _ if ident == "NappguiReal"|| ident == "f32" => Some(Literal::c_string(c"float32_t")),
        _ if ident == "NappguiBoolean"|| ident == "bool" => Some(Literal::c_string(c"bool_t")),
        _ => None,
    }
}
