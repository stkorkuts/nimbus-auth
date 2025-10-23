#[macro_export]
macro_rules! define_enum {
    (
        pub enum $name:ident {
            $($variant:ident),+ $(,)?
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name {
            $($variant),+
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match self {
                    $(Self::$variant => write!(f, stringify!($variant)),)+
                }
            }
        }

        impl ::core::convert::TryFrom<&str> for $name {
            type Error = String;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                match value {
                    $(
                        stringify!($variant) => Ok(Self::$variant),
                    )+
                    _ => Err(format!("invalid value for {} enum. supported values are: {}", stringify!($name), vec![$(stringify!($variant)),+].join(", ")))
                }
            }
        }
    };
}
