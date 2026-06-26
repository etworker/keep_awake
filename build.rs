fn main() {
    #[cfg(windows)]
    {
        let out = std::env::var("OUT_DIR").unwrap();

        // Generate icon.ico via ico crate
        let ico_path = std::path::Path::new(&out).join("icon.ico");
        let rgba = make_sun_rgba();
        let img = ico::IconImage::from_rgba_data(32, 32, rgba);
        let mut dir = ico::IconDir::new(ico::ResourceType::Icon);
        dir.add_entry(ico::IconDirEntry::encode_as_bmp(&img).unwrap());
        dir.write(std::fs::File::create(&ico_path).unwrap()).unwrap();

        // Generate .rc file
        let escaped = ico_path.to_string_lossy().replace('\\', "\\\\");
        std::fs::write(
            std::path::Path::new(&out).join("app.rc"),
            format!("1 ICON \"{}\"\n", escaped),
        )
        .unwrap();

        // windres -> .res (COFF object)
        let res = std::path::Path::new(&out).join("app.res");
        std::process::Command::new("windres")
            .arg(std::path::Path::new(&out).join("app.rc"))
            .arg("-O")
            .arg("coff")
            .arg("-o")
            .arg(&res)
            .status()
            .unwrap();

        // Link the .res
        println!("cargo:rustc-link-arg={}", res.display());
    }
}

#[cfg(windows)]
fn make_sun_rgba() -> Vec<u8> {
    let mut rgba = vec![0u8; 32 * 32 * 4];
    let rays: [(f64, f64); 8] = [
        (1.0, 0.0), (0.707, 0.707), (0.0, 1.0), (-0.707, 0.707),
        (-1.0, 0.0), (-0.707, -0.707), (0.0, -1.0), (0.707, -0.707),
    ];
    for py in 0..32 {
        for px in 0..32 {
            let dx = px as f64 - 15.5;
            let dy = py as f64 - 15.5;
            let dist = (dx * dx + dy * dy).sqrt();
            let i = ((py * 32 + px) * 4) as usize;
            if dist <= 8.0 {
                rgba[i] = 255; rgba[i + 1] = 183; rgba[i + 2] = 77;
                rgba[i + 3] = if dist > 7.0 { ((8.0 - dist).max(0.0) * 255.0) as u8 } else { 255 };
            } else if dist > 9.0 && dist <= 13.0 {
                for (rx, ry) in &rays {
                    let perp = (dx * ry - dy * rx).abs();
                    let proj = dx * rx + dy * ry;
                    if perp <= 1.8 && proj >= 9.0 && proj <= 13.0 {
                        rgba[i] = 255; rgba[i + 1] = 183; rgba[i + 2] = 77;
                        rgba[i + 3] = if perp > 1.0 { ((1.8 - perp).max(0.0) * 255.0) as u8 } else { 255 };
                        break;
                    }
                }
            }
        }
    }
    rgba
}
