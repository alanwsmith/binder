use crate::config::Config;
use crate::file_id::file_id;
use crate::filter_extensions::filter_extensions;
use crate::filter_site::filter_site;
use crate::filter_status::filter_status;
use crate::output_dir_name::output_dir_name;
use crate::override_path::override_path;
use crate::valid_nonce::valid_nonce;
use std::fs::copy;
use std::fs::create_dir_all;
use std::fs::read_dir;
use std::fs::read_to_string;
use std::path::PathBuf;

// This loads files into the side, and
// also exports code blocks to their specified
// file locations
pub fn load_content_from_grimoire() {
    println!("Running load_site");

    // REMEMBER TO CHANGE FILE EXTENSION BELOW ONCE YOU'RE
    // USING .neo FILES
    let prod = Config {
        input_root: "/Users/alan/Grimoire".to_string(),
        output_root: "/Users/alan/workshop/alanwsmith.com/content".to_string(),
        output_sub_dir: Some("pages".to_string()),
        site_id: "aws".to_string(),
    };

    let config = prod.clone();
    let paths = filter_extensions(
        read_dir(&config.input_root)
            .unwrap()
            .into_iter()
            .map(|p| p.expect("here").path())
            .collect::<Vec<PathBuf>>(),
    );

    paths.iter().for_each(|p| {
        // dbg!(&p);
        let data = read_to_string(p).unwrap();
        match (
            filter_status(data.as_str()).unwrap().1,
            filter_site(data.as_str(), config.site_id.as_str())
                .unwrap()
                .1,
            valid_nonce(p.to_path_buf()),
        ) {
            (true, true, true) => {
                let the_file_id = file_id(data.as_str()).unwrap().1.to_string();
                let output_dir_path = PathBuf::from(&config.output_root);
                let mut output_file_path = output_dir_path.clone();
                match override_path(data.as_str()).unwrap().1 {
                    Some(path) => {
                        if let Some(adjust_path) = path.strip_prefix("/") {
                            output_file_path.push(adjust_path);
                        } else {
                            output_file_path.push(path);
                        }
                    }
                    None => {
                        match &config.output_sub_dir {
                            Some(dir) => {
                                output_file_path.push(dir);
                            }
                            None => {}
                        }
                        output_file_path.push(
                            output_dir_name(
                                p.file_stem().unwrap().to_str().unwrap(),
                                the_file_id.as_str(),
                            )
                            .unwrap()
                            .1,
                        );
                        output_file_path.push("index.neo");
                    }
                }
                match output_file_path.parent() {
                    Some(path) => match path.try_exists() {
                        Ok(check) => {
                            // dbg!(&path);
                            if check == false {
                                create_dir_all(path).unwrap();
                            }
                        }
                        Err(_) => {}
                    },
                    None => {}
                }
                // println!("Copying to: {}", &output_file_path.display());
                // dbg!(&output_file_path);
                let _ = copy(p, output_file_path);
            }
            _ => { // dbg!("skipping");
            }
        }
        ()
    });

    println!("Process complete");
}
