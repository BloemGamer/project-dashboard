#[macro_export]
macro_rules! for_each_true
{
    ($flags:expr, $f:expr, $( $field:ident ),*) =>
    {
        $(
            if $flags.$field {
                $f(stringify!($field));
            }
        )*
    };
}

#[macro_export]
macro_rules! generate_path {
    ($base_path:expr, $field:ident) => {{
        let mut path = $base_path;
        path.push(format!("{}.toml", stringify!($field)));
        path
    }};
}

#[macro_export]
macro_rules! get_files
{
    ($cli:expr, $base_path:expr, $data:expr, $( $field:ident => $type:ty => $variant:ident ),* $(,)?) =>
    {{
        $(
            if $cli.$field.is_some()
            {
                let path = generate_path!($base_path.clone(), $field);
                crate::files::ensure_file_exists(&path).expect("Could not find or make file");
                
                use fs;
                let toml_file: String = fs::read_to_string(path).expect("Could not read file");
                $data.$field = toml::from_str(&toml_file).unwrap();
            };
        )*

    }};
}

//#[macro_export]
//macro_rules! generate_load_futures {
//    ($cli:expr, $base_path:expr, $( $field:ident => $type:ty => $variant:ident ),* $(,)?) => {{
//        let mut futures = Vec::new();
//
//        $(
//            if $cli.$field.is_some() {
//                let path = generate_path!($base_path.clone(), $field);
//
//                let fut = async move {
//                    match tokio::fs::read_to_string(&path).await {
//                        Ok(content) => match toml::from_str::<$type>(&content) {
//                            Ok(parsed) => Ok(DataEnum::$variant(parsed)),
//                            Err(e) => Err(format!("TOML error in {}: {}", path.display(), e)),
//                        },
//                        Err(e) => Err(format!("File read error in {}: {}", path.display(), e)),
//                    }
//                };
//
//                futures.push(fut);
//            }
//        )*
//
//        futures
//    }};
//}

//#[macro_export] macro_rules! generate_data_enum
//{
//    (
//        $(#[$attr:meta])*
//        $vis:vis struct $struct_name:ident
//        {
//            $(
//                $field_vis:vis $field:ident : Option<$ty:ty> => $variant:ident,
//            )*
//        }
//    ) => {
//        // Generate the struct with attributes
//        $(#[$attr])*
//        $vis struct $struct_name
//        {
//            $(
//                $field_vis $field: Option<$ty>,
//            )*
//        }
//
//        // Generate the enum
//        $vis enum DataEnum
//        {
//            $(
//                $variant($ty),
//            )*
//        }
//    };
//}
