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
    ($base_path:expr, $data:expr, $( $field:ident => $type:ty => $default:expr),* $(,)?) =>
    {{
        $(
            let path: std::path::PathBuf = generate_path!($base_path.clone(), $field);
            crate::files::ensure_file_exists(&path).expect("Could not find or make file");
            
            let toml_file: String = std::fs::read_to_string(path).expect("Could not read file");
            $data.$field = toml::from_str(&toml_file).unwrap_or_else(|_| $default);
        )*

    }};
}

#[macro_export]
macro_rules! draw_terminal
{
    ($terminal:expr => $render_function:ident ( $($args:expr),*): $app_state:expr, $data:expr) =>
    {{
        if !$app_state.has_error()
        {
            $terminal.draw(|frame| $render_function(frame $(, $args)*)).unwrap();
        } else {
            $terminal.draw(|frame|
            {
                $render_function(frame $(, $args)*);
                crate::tui::render_log_popup(frame, $app_state.error_state.as_ref().unwrap(), &$data.settings.colors)
            }).unwrap();
        }
    }};
}

