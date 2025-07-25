use anyhow::Result;
use headless_chrome::{Browser, protocol::cdp::Page::CaptureScreenshotFormatOption};
use std::path::{self, Path};

pub fn render_html(browser: &Browser, html_file: &Path) -> Result<Vec<u8>> {
    let tab = browser.new_tab()?;

    let viewport = tab
        .navigate_to(&path::absolute(html_file)?.to_string_lossy())?
        .wait_for_element("div.post")?
        .get_box_model()?
        .border_viewport();

    Ok(tab.capture_screenshot(
        CaptureScreenshotFormatOption::Png,
        None,
        Some(viewport),
        true,
    )?)
}
