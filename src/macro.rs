
#[macro_export]
macro_rules! generate_path
{
    ($base_path:expr, $field:ident) =>
    {{
        let mut path = $base_path;
        path.push(format!("{}.toml", stringify!($field)));
        path
    }};
}

#[macro_export]
macro_rules! get_files
{
    ($base_path:expr, $data:expr, $( $field:ident => $type:ty),* $(,)?) =>
    {{
        $(
            let path: std::path::PathBuf = generate_path!($base_path.clone(), $field);
            crate::files::ensure_file_exists(&path).expect("Could not find or make file");
            
            let toml_file: String = std::fs::read_to_string(path).expect("Could not read file");
            $data.$field = toml::from_str(&toml_file).unwrap();
        )*

    }};
}
