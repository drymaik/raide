use ron::ser::PrettyConfig;
/// Ron prettifier
pub fn get_pretty() -> PrettyConfig {
    PrettyConfig {
        depth_limit: 2,
        new_line: "\n".to_string(),
        indentor: "    ".to_string(),
        separate_tuple_members: true,
        enumerate_arrays: true,
    }
}
