use core::fmt::Error;
use miette::{IntoDiagnostic, Result};
use site_loader::load_content_from_grimoire::load_content_from_grimoire;
use site_loader::update_meilisearch::update_search_document;
use std::process::Command;
use std::time::Duration;
use watchexec::{
    action::{Action, Outcome},
    config::{InitConfig, RuntimeConfig},
    handler::PrintDebug,
    Watchexec,
};
use watchexec_events::Tag;
use watchexec_signals::Signal;

// This is currently hard coded to look for:
// `>> site: neoengine``
//
// Note this is current etup to look at .org files
// until the move to raw .neo files in the grimoire is made
//
// [] Move nonce words and paths into config file
// [] Only allows specific nonce- words
// [] Add a note to the top of the files saying that they are copies
// [] Handle dir paths if there is no id
// and should not be edited?. Point to full path
//  to help prevenet editing the wrong ones
// [x] Handle explicit paths
// [x] Configure site directory and subdirectory paths independently
// [x] Only allows .neo extensions
// [x] Only allows published and draft files
// [x] Make direcotires if necessary
// [x] Ouput goes into a sub directory if one is defined
// [x] Each file has its own directory created for it
// [x] The output file is always index.neo in the folder
// [x] Strip `.` from url

#[tokio::main]
async fn main() -> Result<()> {
    try_to_set_tmux_title();
    load_content_from_grimoire();
    let mut init = InitConfig::default();
    init.on_error(PrintDebug(std::io::stderr()));
    let mut runtime = RuntimeConfig::default();
    runtime.pathset(["/Users/alan/Grimoire"]);
    runtime.action_throttle(Duration::new(0, 100000000));
    let we = Watchexec::new(init, runtime.clone())?;
    runtime.on_action(move |action: Action| async move {
        let mut stop_running = false;
        let mut load_content = false;
        for event in action.events.iter() {
            event.signals().for_each(|sig| match sig {
                Signal::Interrupt => {
                    stop_running = true;
                }
                _ => {}
            });
            if event
                .paths()
                .any(|(p, _)| p.starts_with("/Users/alan/Grimoire"))
            {
                let path_thing = &event.tags.iter().find_map(|t| match t {
                    Tag::Path { path, .. } => Some(path),
                    _ => None,
                });

                match path_thing.unwrap().extension() {
                    Some(e) => match e.to_str().unwrap() {
                        "org" => {
                            // dbg!(path_thing);
                            update_search_document(path_thing.unwrap().to_str().unwrap())
                        }
                        _ => {}
                    },
                    None => {}
                }

                load_content = true;
            }
        }
        if stop_running {
            action.outcome(Outcome::Exit);
        }
        if load_content {
            load_content_from_grimoire();
        }
        Ok::<(), Error>(())
    });
    let _ = we.reconfigure(runtime);
    let _ = we.main().await.into_diagnostic()?;
    Ok(())
}

pub fn try_to_set_tmux_title() {
    let args: Vec<&str> = vec!["select-pane", "-T", "Site Loader"];
    let _ = Command::new("tmux").args(args).output().unwrap();
}
