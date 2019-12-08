#[macro_export]
macro_rules! load_input {
    () => (load_input!(""));
    ($separator:expr) => (load_input!($separator, String::from));
    ($separator:expr, $ty:ident) => (load_input!($separator, |n| n.parse::<$ty>().unwrap()));
    ($separator:expr, $( $mapper:expr ),*) => {{
        let mut input = std::string::String::new();
        std::io::Read::read_to_string(&mut std::io::stdin(), &mut input)
            .expect("unable to read stdin");

        input
            .split($separator)
            .map(|l| l.replace(&['\n'][..], ""))
            .filter(|l| l.len() > 0)
            $(.map($mapper))*
            .collect()
    }};
}
