use bitvec::bitvec;
use bitvec::order::Msb0;
use bitvec::prelude::BitVec;
use headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption;
use headless_chrome::protocol::cdp::Target::CreateTarget;
use headless_chrome::{Browser, LaunchOptions};
use image::{GenericImageView, Pixel};
use std::io::{Cursor, Write};
use log::warn;

pub fn render_html(content: String, width: u32, height: u32) -> anyhow::Result<Vec<u8>> {
    let browser = Browser::new(LaunchOptions {
        window_size: Some((1920, 1080)),
        ..Default::default()
    })?;

    // Open a new tab
    let tab = browser.new_tab_with_options(CreateTarget {
        url: "about:blank".to_string(),
        width: Some(1920),
        height: Some(1080),
        browser_context_id: None,
        enable_begin_frame_control: None,
        new_window: None,
        background: None,
    })?;

    // Load HTML content (can be a local file or a data URL)
    let data_url = format!("data:text/html,{}", urlencoding::encode(content.as_str()));
    tab.navigate_to(&data_url)?.wait_until_navigated()?;

    tab.wait_for_element(".screen")?;

    let element = tab.find_element(".screen")?;
    element.scroll_into_view()?;

    let box_model = element.get_box_model()?;
    let mut viewport = box_model.margin_viewport();
    // default template renders at 800x480
    //viewport.width = width as f64;
    //viewport.height = height as f64;
    //viewport.scale = (width as f32 / 800.0) as JsFloat;
    warn!("Viewport: {:?}", viewport);

    let screenshot = tab.capture_screenshot(
        CaptureScreenshotFormatOption::Png,
        Some(100),
        Some(viewport),
        true,
    )?;
    // Convert PNG to BMP
    let img = image::load_from_memory(&screenshot)?;
    // Resize to 800x480
    let img = img.resize_exact(width, height, image::imageops::FilterType::Gaussian);

    // Convert to 1-bit
    let threshold = 128u8;
    let mut bw_bits = bitvec![u8, Msb0; 0; img.width() as usize * img.height() as usize];

    for (i, pixel) in img.pixels().enumerate() {
        let (_, _, p) = pixel;

        bw_bits.set(i, p.to_luma()[0] >= threshold);
    }

    // Save as 1-bit BMP
    let buffer = save_1bit_bmp(img.width(), img.height(), &bw_bits)?;

    drop(browser);

    Ok(buffer)
}

fn save_1bit_bmp(width: u32, height: u32, bits: &BitVec<u8, Msb0>) -> anyhow::Result<Vec<u8>> {
    let row_size = ((width + 7) / 8) as usize;
    let padding = (4 - (row_size % 4)) % 4; // BMP rows must be 4-byte aligned
    let image_size = (row_size + padding) * height as usize;
    let file_size = 54 + 8 + image_size; // Header + Palette + Image Data

    let mut buffer = Cursor::new(Vec::with_capacity(file_size));

    // BMP Header
    buffer.write_all(b"BM")?;
    buffer.write_all(&(file_size as u32).to_le_bytes())?;
    buffer.write_all(&[0; 4])?; // Reserved
    buffer.write_all(&62u32.to_le_bytes())?; // Offset to image data

    // DIB Header
    buffer.write_all(&40u32.to_le_bytes())?; // Header size
    buffer.write_all(&(width as i32).to_le_bytes())?;
    buffer.write_all(&(height as i32).to_le_bytes())?; // Negative to store top-down
    buffer.write_all(&1u16.to_le_bytes())?; // Color planes
    buffer.write_all(&1u16.to_le_bytes())?; // 1 bit per pixel
    buffer.write_all(&0u32.to_le_bytes())?; // No compression
    buffer.write_all(&(image_size as u32).to_le_bytes())?; // Image size
    buffer.write_all(&[0; 16])?; // Resolution & color info (ignored)

    // Color Palette (Black & White)
    buffer.write_all(&[0, 0, 0, 0, 255, 255, 255, 0])?;

    // Image Data
    for y in (0..height).rev() {
        let start = (y * width) as usize;
        let row = &bits[start..start + width as usize];

        let mut packed_row = vec![0u8; row_size];
        for (i, bit) in row.iter().enumerate() {
            if *bit {
                packed_row[i / 8] |= 1 << (7 - (i % 8));
            }
        }

        buffer.write_all(&packed_row)?;
        buffer.write_all(&vec![0u8; padding])?;
    }

    Ok(buffer.into_inner())
}
