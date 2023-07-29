#![allow(unused_imports)]
use crate::password::pw;
use futures::executor::block_on;
use meilisearch_sdk::client::*;
use serde::{Deserialize, Serialize};
use slug::slugify;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
struct Doc {
    file_name: String,
    title: String,
    id: String,
    contents: String,
    facets: Vec<String>,
}

pub fn update_search_document(file_path: &str) {
    block_on(async move {
        let passkey = pw("alan--meilisearch--scratchpad--admin-key", "alan");
        let client = Client::new("http://127.0.0.1:7700", Some(passkey));

        let grimoire = client.index("grimoire");
        let contents = fs::read_to_string(file_path);
        let file_name = PathBuf::from(file_path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let d = Doc {
            file_name: file_name.clone(),
            title: "".to_string(),
            id: slugify(&file_name),
            facets: vec![],
            contents: contents.as_ref().unwrap().to_string(),
        };

        let _ = grimoire
            .add_documents(&[d], None)
            .await
            .unwrap()
            .wait_for_completion(&client, None, None)
            .await
            .unwrap();

        //
    })
}
