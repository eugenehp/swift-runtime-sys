use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

/// Derive macro for SwiftUI view components.
///
/// Fields marked `#[state]` become reactive state values.
/// The struct must implement a `body(&self, cx: &Cx) -> View` method.
///
/// ```ignore
/// #[derive(View)]
/// struct Counter {
///     #[state] count: i32,
///     label: String, // prop, not state
/// }
///
/// impl Counter {
///     fn body(&self, cx: &Cx) -> View {
///         vstack![
///             text(&format!("{}: {}", self.label, self.count)).size(24.0),
///             button("+1", cx.bind(&self.count, |n| n + 1)),
///         ]
///     }
/// }
///
/// // Usage:
/// app("App", 400.0, 300.0, |cx| {
///     Counter::new(cx, "My Counter".to_string()).render(cx)
/// });
/// ```
#[proc_macro_derive(View, attributes(state))]
pub fn derive_view(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("#[derive(View)] only supports named fields"),
        },
        _ => panic!("#[derive(View)] only supports structs"),
    };

    let mut state_fields = Vec::new();
    let mut prop_fields = Vec::new();

    for field in fields {
        let ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        let is_state = field.attrs.iter().any(|a| a.path().is_ident("state"));

        if is_state {
            state_fields.push((ident.clone(), ty.clone()));
        } else {
            prop_fields.push((ident.clone(), ty.clone()));
        }
    }

    // Generate the struct with State<T> fields
    let state_field_defs: Vec<_> = state_fields
        .iter()
        .map(|(ident, ty)| {
            quote! { pub #ident: swiftui::state::State<#ty> }
        })
        .collect();

    let prop_field_defs: Vec<_> = prop_fields
        .iter()
        .map(|(ident, ty)| {
            quote! { pub #ident: #ty }
        })
        .collect();

    // Generate new() constructor
    let prop_params: Vec<_> = prop_fields
        .iter()
        .map(|(ident, ty)| {
            quote! { #ident: #ty }
        })
        .collect();

    let state_inits: Vec<_> = state_fields
        .iter()
        .map(|(ident, ty)| {
            quote! { #ident: cx.state(<#ty>::default()) }
        })
        .collect();

    let prop_inits: Vec<_> = prop_fields
        .iter()
        .map(|(ident, _)| {
            quote! { #ident }
        })
        .collect();

    // Generate view() method that reads state into a snapshot struct
    let state_reads: Vec<_> = state_fields
        .iter()
        .map(|(ident, _)| {
            quote! { #ident: self.#ident.get() }
        })
        .collect();

    let prop_copies: Vec<_> = prop_fields
        .iter()
        .map(|(ident, _)| {
            quote! { #ident: self.#ident.clone() }
        })
        .collect();

    // The generated wrapper struct name
    let wrapper_name = Ident::new(&format!("{name}Component"), name.span());

    let expanded = quote! {
        pub struct #wrapper_name {
            #(#state_field_defs,)*
            #(#prop_field_defs,)*
        }

        impl #wrapper_name {
            pub fn new(cx: &swiftui::state::Cx, #(#prop_params),*) -> Self {
                Self {
                    #(#state_inits,)*
                    #(#prop_inits,)*
                }
            }

            pub fn render(&self, cx: &swiftui::state::Cx) -> swiftui::View {
                // Create a snapshot with current state values
                let snapshot = #name {
                    #(#state_reads,)*
                    #(#prop_copies,)*
                };
                snapshot.body(self, cx)
            }
        }

        impl #name {
            fn bind<T: std::any::Any + Send + Clone + 'static>(
                component: &#wrapper_name,
                state: &swiftui::state::State<T>,
                f: impl Fn(&T) -> T + 'static,
            ) -> impl Fn() + 'static {
                state.bind(f)
            }
        }
    };

    TokenStream::from(expanded)
}

/// Format macro that works with State values.
///
/// ```ignore
/// let count = cx.state(42i32);
/// let name = cx.state("world".to_string());
/// let s = text_fmt!("Hello {name}, count={count}");
/// // Expands to: text(&format!("Hello {}, count={}", name.get(), count.get()))
/// ```
#[proc_macro]
pub fn text_fmt(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as syn::LitStr);
    let val = lit.value();

    // Find {ident} patterns
    let mut fmt_str = String::new();
    let mut args = Vec::new();
    let mut chars = val.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '{' {
            if chars.peek() == Some(&'{') {
                chars.next();
                fmt_str.push_str("{{");
                continue;
            }
            let mut ident = String::new();
            while let Some(&c) = chars.peek() {
                if c == '}' {
                    chars.next();
                    break;
                }
                ident.push(c);
                chars.next();
            }
            fmt_str.push_str("{}");
            let id = Ident::new(&ident, proc_macro2::Span::call_site());
            args.push(quote! { #id.get() });
        } else if c == '}' {
            if chars.peek() == Some(&'}') {
                chars.next();
                fmt_str.push_str("}}");
            } else {
                fmt_str.push(c);
            }
        } else {
            fmt_str.push(c);
        }
    }

    let expanded = if args.is_empty() {
        quote! { swiftui::dsl::text(#val) }
    } else {
        quote! { swiftui::dsl::text(&format!(#fmt_str, #(#args),*)) }
    };

    TokenStream::from(expanded)
}
