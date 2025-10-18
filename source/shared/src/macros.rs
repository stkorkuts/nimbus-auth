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
    };
}
