use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    FnArg, GenericArgument, ItemTrait, Pat, PathArguments, ReturnType, TraitItem, TraitItemFn,
    Type, parse_macro_input, parse_quote, punctuated::Punctuated, token::Comma,
};

/// Helper function to extract the Output type from an `impl Future<Output = T>` return type
fn extract_future_output_type(ty: &Type) -> Option<Type> {
    // Try to extract from `impl Future<Output = T> + Send`
    if let Type::ImplTrait(impl_trait) = ty {
        for bound in &impl_trait.bounds {
            if let syn::TypeParamBound::Trait(trait_bound) = bound {
                if trait_bound.path.segments.len() == 1
                    && trait_bound.path.segments[0].ident == "Future"
                {
                    // Extract the Output type from the Future<Output = T>
                    if let PathArguments::AngleBracketed(args) =
                        &trait_bound.path.segments[0].arguments
                    {
                        for arg in &args.args {
                            // In newer syn versions, it's AssocType, not Binding
                            if let GenericArgument::AssocType(assoc_type) = arg {
                                if assoc_type.ident == "Output" {
                                    return Some(assoc_type.ty.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // If we can't extract it, default to ()
    Some(parse_quote!(()))
}

/// Helper function to get method arguments as a comma-separated list for calling
fn get_arg_names(inputs: &Punctuated<FnArg, Comma>) -> Vec<proc_macro2::TokenStream> {
    inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Receiver(_) => Some(quote! { self }),
            FnArg::Typed(pat_type) => {
                if let Pat::Ident(pat_ident) = &*pat_type.pat {
                    let arg_name = &pat_ident.ident;
                    Some(quote! { #arg_name })
                } else {
                    None
                }
            }
        })
        .collect()
}

/// Procedural macro to automatically generate a dynamic dispatch version of an async trait
///
/// # Example
/// ```rust
/// #[auto_dynamic]
/// trait AsyncTrait {
///     fn do_async(&self) -> impl Future<Output = ()> + Send;
/// }
/// ```
#[proc_macro_attribute]
pub fn make_dynamic(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemTrait);
    let trait_name = &input.ident;
    let dyn_trait_name = format_ident!("{}Dyn", trait_name);

    let vis = &input.vis;
    let generics = &input.generics;
    let where_clause = &input.generics.where_clause;

    // Extract trait methods
    let methods: Vec<&TraitItemFn> = input
        .items
        .iter()
        .filter_map(|item| {
            if let TraitItem::Fn(method) = item {
                Some(method)
            } else {
                None
            }
        })
        .collect();

    // Generate async methods for the dynamic trait
    let dyn_methods = methods.iter().map(|method| {
        let original_method_name = &method.sig.ident;
        let method_name = format_ident!("{}_dyn", original_method_name);
        let inputs = &method.sig.inputs;
        let attrs = &method.attrs;

        // Extract the output type from `impl Future<Output = T>`
        let output_type = match &method.sig.output {
            ReturnType::Default => quote! { () },
            ReturnType::Type(_, ty) => {
                if let Some(future_output) = extract_future_output_type(ty) {
                    quote! { #future_output }
                } else {
                    quote! { () }
                }
            }
        };

        quote! {
            #(#attrs)*
            async fn #method_name(#inputs) -> #output_type;
        }
    });

    // Generate implementation of the dynamic trait for types implementing the original trait
    let impl_methods = methods.iter().map(|method| {
        let original_method_name = &method.sig.ident;
        let method_name = format_ident!("{}_dyn", original_method_name);
        let inputs = &method.sig.inputs;
        let arg_names = get_arg_names(inputs);

        // Extract the output type from `impl Future<Output = T>`
        let output_type = match &method.sig.output {
            ReturnType::Default => quote! { () },
            ReturnType::Type(_, ty) => {
                if let Some(future_output) = extract_future_output_type(ty) {
                    quote! { #future_output }
                } else {
                    quote! { () }
                }
            }
        };

        quote! {
            async fn #method_name(#inputs) -> #output_type {
                #trait_name::#original_method_name(#(#arg_names),*).await
            }
        }
    });

    // Instead of including the original trait in the output, we only generate the dynamic trait
    let expanded = quote! {
        #[async_trait::async_trait]
        #vis trait #dyn_trait_name #generics #where_clause {
            #(#dyn_methods)*
        }

        #[async_trait::async_trait]
        impl<T: #trait_name + Sync> #dyn_trait_name for T #where_clause {
            #(#impl_methods)*
        }
    };

    // We're going to return the original trait definition plus the dynamic trait
    quote! {
        #input

        #expanded
    }
    .into()
}

/// Procedural macro to automatically generate a thread-local version of an async trait
///
/// # Example
/// ```rust
/// #[auto_local]
/// trait AsyncTrait {
///     fn do_async(&self) -> impl Future<Output = ()> + Send;
/// }
/// ```
#[proc_macro_attribute]
pub fn make_local(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemTrait);
    let trait_name = &input.ident;
    let local_trait_name = format_ident!("{}Local", trait_name);

    let vis = &input.vis;
    let generics = &input.generics;
    let where_clause = &input.generics.where_clause;

    // Extract trait methods
    let methods: Vec<&TraitItemFn> = input
        .items
        .iter()
        .filter_map(|item| {
            if let TraitItem::Fn(method) = item {
                Some(method)
            } else {
                None
            }
        })
        .collect();

    // Generate async methods for the local trait
    let local_methods = methods.iter().map(|method| {
        let original_method_name = &method.sig.ident;
        let method_name = format_ident!("{}_local", original_method_name);
        let inputs = &method.sig.inputs;
        let attrs = &method.attrs;

        // Extract the output type from `impl Future<Output = T>`
        let output_type = match &method.sig.output {
            ReturnType::Default => quote! { () },
            ReturnType::Type(_, ty) => {
                if let Some(future_output) = extract_future_output_type(ty) {
                    quote! { #future_output }
                } else {
                    quote! { () }
                }
            }
        };

        quote! {
            #(#attrs)*
            async fn #method_name(#inputs) -> #output_type;
        }
    });

    // Generate implementation of the local trait for types implementing the original trait
    let impl_methods = methods.iter().map(|method| {
        let original_method_name = &method.sig.ident;
        let method_name = format_ident!("{}_local", original_method_name);
        let inputs = &method.sig.inputs;
        let arg_names = get_arg_names(inputs);

        // Extract the output type from `impl Future<Output = T>`
        let output_type = match &method.sig.output {
            ReturnType::Default => quote! { () },
            ReturnType::Type(_, ty) => {
                if let Some(future_output) = extract_future_output_type(ty) {
                    quote! { #future_output }
                } else {
                    quote! { () }
                }
            }
        };

        quote! {
            async fn #method_name(#inputs) -> #output_type {
                #trait_name::#original_method_name(#(#arg_names),*).await
            }
        }
    });

    // Instead of including the original trait in the output, we only generate the local trait
    let expanded = quote! {
        #[async_trait::async_trait(?Send)]
        #vis trait #local_trait_name #generics #where_clause {
            #(#local_methods)*
        }

        #[async_trait::async_trait(?Send)]
        impl<T: #trait_name> #local_trait_name for T #where_clause {
            #(#impl_methods)*
        }
    };

    // We're going to return the original trait definition plus the local trait
    quote! {
        #input

        #expanded
    }
    .into()
}
