pub(crate) fn camel_to_snake_case(camel_case: &str) -> String {
    let mut snake_case = String::new();
    let mut prev_char: Option<char> = None;

    for c in camel_case.chars() {
        if let Some(_) = prev_char {
            if c.is_uppercase() {
                snake_case.push('_');
                snake_case.push(c.to_lowercase().next().unwrap());
            } else {
                snake_case.push(c);
            }
            prev_char = Some(c);
        } else {
            snake_case.push(c.to_lowercase().next().unwrap());
            prev_char = Some(c);
        }
    }

    snake_case
}

#[macro_export]
macro_rules! execute_if {
    ($cond:expr, $expr:expr) => {
        if $cond {
            $expr
        }
    };
}

#[macro_export]
macro_rules! enum_with_to_string {
    (pub enum $name:ident {
        $($variant:ident),*,
    }) => {
        pub enum $name {
            $($variant),*
        }

        impl $name {
            pub fn to_string(&self) -> String {
                match self {
                    $($name::$variant => camel_to_snake_case(stringify!($variant))),*
                }
            }
        }

        impl std::str::FromStr for $name {
            type Err = CliError;

            fn from_str(s: &str) -> Result<Self> {
                $(execute_if!(s == camel_to_snake_case(stringify!($variant)), return Ok($name::$variant));)*
                return Err(CliError::StringError(format!("invalid variant: {s}")));
            }
        }
    };
}