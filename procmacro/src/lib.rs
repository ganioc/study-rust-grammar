extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn inspect(arguments: TokenStream, item: TokenStream) -> TokenStream {
    println!("arguments: \"{}\"", arguments.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}

#[proc_macro]
pub fn print_info(_item: TokenStream) -> TokenStream{
    "fn print_info(){ println!(\"Hello!\");}".parse().unwrap()
}

#[proc_macro_derive(HelloFn)]
pub fn derive_hellofn(item: TokenStream) -> TokenStream{
    let parsed_input = item.to_string().replace("struct", "").replace(";","");
    let impl_ = format!("impl {} {{
            pub fn hello(&self) -> String {{\"Hello\".to_string()}}
        }}", parsed_input);
    impl_.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        println!("test procmacro it_works!");

    }
}
