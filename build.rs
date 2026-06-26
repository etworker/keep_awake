fn main() {
    #[cfg(windows)]
    {
        let out = std::env::var("OUT_DIR").unwrap();

        // Generate ICO file with sun icon
        let ico_path = std::path::Path::new(&out).join("keep-awake.ico");
        std::fs::write(&ico_path, make_ico_bytes()).unwrap();

        // Embed via Windows Resource
        winres::WindowsResource::new()
            .set_icon(&ico_path.to_string_lossy())
            .compile()
            .unwrap();
    }
}

#[cfg(windows)]
fn make_ico_bytes() -> Vec<u8> {
    const W: u32 = 32;
    const H: u32 = 32;

    // Generate RGBA sun pixels (same design as tray.rs)
    let mut rgba = vec![0u8; (W * H * 4) as usize];
    let rays: [(f64, f64); 8] = [
        (1.0, 0.0), (0.707, 0.707), (0.0, 1.0), (-0.707, 0.707),
        (-1.0, 0.0), (-0.707, -0.707), (0.0, -1.0), (0.707, -0.707),
    ];
    for py in 0..H {
        for px in 0..W {
            let dx = px as f64 - 15.5;
            let dy = py as f64 - 15.5;
            let dist = (dx * dx + dy * dy).sqrt();
            let i = ((py * W + px) * 4) as usize;
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

    // Pack into ICO format (BMP-based, bottom-up rows, BGRA)
    let bih_size = 40u32;
    let xor_size = W * H * 4;
    let and_size = W * H / 8;
    let img_size = bih_size + xor_size + and_size;
    let file_size = 6 + 16 + img_size;

    let mut buf = Vec::with_capacity(file_size as usize);

    // ICO header
    buf.extend_from_slice(&[0, 0, 1, 0, 1, 0]);

    // Directory entry
    buf.push(32); // w
    buf.push(32); // h
    buf.push(0);  // colors
    buf.push(0);  // reserved
    buf.extend_from_slice(&[1, 0]); // planes
    buf.extend_from_slice(&[32, 0]); // bpp
    buf.extend_from_slice(&img_size.to_le_bytes());
    buf.extend_from_slice(&22u32.to_le_bytes()); // offset = 6 + 16

    // BITMAPINFOHEADER
    buf.extend_from_slice(&bih_size.to_le_bytes()); // size
    buf.extend_from_slice(&W.to_le_bytes());        // width
    buf.extend_from_slice(&(H * 2).to_le_bytes());  // height (double for AND mask)
    buf.extend_from_slice(&[1, 0]);                  // planes
    buf.extend_from_slice(&[32, 0]);                 // bpp
    buf.extend_from_slice(&[0; 16]);                 // compression, etc.
    buf.extend_from_slice(&[0; 8]);                  // colors

    // XOR mask (BGRA, bottom-up: write last row first)
    for y in (0..H).rev() {
        for x in 0..W {
            let src = ((y * W + x) * 4) as usize;
            buf.push(rgba[src + 2]); // B
            buf.push(rgba[src + 1]); // G
            buf.push(rgba[src]);     // R
            buf.push(rgba[src + 3]); // A
        }
    }

    // AND mask (all zeros = fully opaque / uses alpha channel)
    buf.extend(std::iter::repeat(0u8).take(and_size as usize));

    buf
}
