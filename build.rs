fn main() {
    println!("cargo:warning=BUILD SCRIPT STARTED"); // Это сообщение вы увидите при сборке
    
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        
        println!("cargo:warning=EMBEDDING ICON...");
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        
        // Важно: не игнорируйте ошибку, а выведите её!
        if let Err(e) = res.compile() {
            println!("cargo:warning=ICON COMPILE FAILED: {}", e);
        }
    }
}