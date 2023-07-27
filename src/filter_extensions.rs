use std::path::PathBuf;

pub fn filter_extensions(list: Vec<PathBuf>) -> Vec<PathBuf> {
    list.into_iter()
        .filter(|p| match p.extension() {
            Some(ext) => {
                if ext == "org" {
                    true
                } else {
                    false
                }
            }
            None => false,
        })
        .collect()
}
