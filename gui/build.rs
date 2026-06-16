fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let icon_path = "assets/icon.ico";
        if std::path::Path::new(icon_path).exists() {
            let mut res = winres::WindowsResource::new();
            res.set_icon(icon_path);
            res.compile().expect("failed to embed application icon");
        } else {
            println!("cargo:warning=gui/{icon_path} not found, skipping icon embedding");
        }
    }
}
