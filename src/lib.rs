use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, parse2};

#[proc_macro_derive(Fields, attributes(fields))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let input = parse2::<DeriveInput>(input).unwrap();

    // Structs name that has #[derive(Fields)]
    let name = &input.ident;

    // get the struct data out of the input and panic if it's anything else
    let data_struct = get_struct_data(&input);
    let fields = data_struct.fields;

    // get the field names of the struct
    let names = get_field_names(&fields);

    // pass through generics
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    // since `fields` is `const`, we need to know how many names we have
    let len = names.len();

    let expanded = quote! {
         impl #impl_generics #name #ty_generics #where_clause {
            pub const fn fields() -> [&'static str; #len] {
                [#(#names,)*]
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

/// Get the struct data out of the input and panic if it's an enum or union
fn get_struct_data(input: &DeriveInput) -> DataStruct {
    match &input.data {
        Data::Enum(_) => panic!("Fields derive macro is only supported on structs"),
        Data::Union(_) => {
            panic!("Fields derive macro is only supported on structs with named fields")
        }
        Data::Struct(data_struct) => {
            if data_struct.fields.members().any(|member| matches!(member, syn::Member::Unnamed(..)))
            {
                panic!("Unit structs are not supported")
            }
            data_struct.to_owned()
        }
    }
}

/// Get the name of each field as a string
fn get_field_names(fields: &Fields) -> Vec<String> {
    fields
        .into_iter()
        .filter_map(|field| {
            // skip private fields
            if matches!(field.vis, syn::Visibility::Public(..)) {
                field.ident.as_ref().map(|ident| ident.to_string())
            } else {
                None
            }
        })
        .collect()
}
