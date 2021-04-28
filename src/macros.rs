macro_rules! builder {
    ($name:ident {
        $($field:ident: $type:ty,)*
    }) => {
        #[derive(Builder, Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
        #[builder(setter(strip_option))]
        pub struct $name {
            $(
                #[serde(skip_serializing_if = "Option::is_none")]
                #[builder(default)]
                $field: Option<$type>,
            )*
        }
        impl $name {
            pub fn new() -> Self {
                Self::default()
            }
        }
    };
}
