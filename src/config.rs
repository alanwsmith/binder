#[derive(Clone)]
pub struct Config {
    pub output_root: String,
    pub output_sub_dir: Option<String>,
    pub input_root: String,
    pub site_id: String,
}
