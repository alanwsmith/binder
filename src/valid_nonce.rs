use std::path::PathBuf;

pub fn valid_nonce(p: PathBuf) -> bool {
    // TODO: Remove `aws-` to see if there's anytying there that needs
    // to be removed
    // Don't do `cloudinary- ` until you've scrubbed it
    let nonces = vec![
        "alt- ",
        "ansible- ",
        "apis- ",
        "app- ",
        "applescript- ",
        "apps- ",
        "ascii- ",
        "audition- ",
        "automator- ",
        "awk- ",
        "bash- ",
        "bbedit- ",
        "blender- ",
        "bookmarks- ",
        "books- ",
        "chrome- ",
        "classnotes- ",
        "cli- ",
        "colors- ",
        "confnotes- ",
        "css- ",
        "cuc- ",
        "d3- ",
        "daily-links- ",
        "data- ",
        "davinci- ",
        "design- ",
        "dev- ",
        "django- ",
        "docker- ",
        "dotfiles- ",
        "drupal- ",
        "eclipse- ",
        "emacs- ",
        "electron- ",
        "examples- ",
        "exiftool- ",
        "ffmpeg- ",
        "freenas- ",
        "gatsby- ",
        "gif- ",
        "github- ",
        "grim- ",
        "grub- ",
        "hammerspoon-",
        "heroku- ",
        "html- ",
        "htpc- ",
        "httrack- ",
        "hugo- ",
        "iterm2- ",
        "jekyll- ",
        "jq- ",
        "jquery- ",
        "js- ",
        "json- ",
        "keyboard-maestro- ",
        "keyboards- ",
        "kindle- ",
        "launchd- ",
        "ligthroom- ",
        "lists- ",
        "lua- ",
        "image-magick- ",
        "minecraft- ",
        "misc- ",
        "music- ",
        "musicbrainz- ",
        "neo- ",
        "neoe- ",
        "neop- ",
        "netlify- ",
        "nextjs- ",
        "nginx- ",
        "ngrok- ",
        "node- ",
        "nokogiri- ",
        "notes- ",
        "nvalt- ",
        "nvim- ",
        //

        //
        "post- ",
        "review- ",
        "site- ",
        "stream- ",
        "tools- ",
    ];

    match nonces.iter().find(|&&n| {
        p.file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap()
            .starts_with(n)
    }) {
        Some(_) => true,
        None => false,
    }
}
