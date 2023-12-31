pub mod config;

#[macro_export]
macro_rules! create_db_structs {
    ($ident:ident, $insert_name:ident, $($element: ident: $ty: ty),*) => {
        #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
        pub struct $insert_name {
            $(pub $element: $ty),*
        }

        #[derive(Clone, Debug, Default, PartialEq, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
        pub struct $ident {
            pub id: i64,
            $(pub $element: $ty),*
        }
    }
}
