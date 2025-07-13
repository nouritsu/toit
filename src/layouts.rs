use std::{collections::HashMap, fs, path::Path};
use thiserror::Error;

pub struct LayoutHandler {
    pub layouts: HashMap<String, String>,
}

#[derive(Error, Debug)]
pub enum LayoutHandlerError {
    #[error("failed to read layout file(s): {0}")]
    IOError(#[from] std::io::Error),

    #[error("failed to process liquid template: {0}")]
    LiquidError(#[from] liquid::Error),
}

impl LayoutHandler {
    pub fn build(directory: &Path) -> Result<Self, LayoutHandlerError> {
        let mut layouts = HashMap::new();
        let base = fs::read_to_string(directory.join("default.html"))?;
        let template = liquid::ParserBuilder::with_stdlib().build()?.parse(&base)?;

        for entry in fs::read_dir(directory)? {
            let path = entry?.path();

            if !(path.is_file()
                && path.extension().is_some_and(|ext| ext == "html")
                && path.file_name().is_some_and(|name| name != "default.html"))
            {
                continue;
            }

            let data = fs::read_to_string(&path)?;
            let globals = liquid::object!({
                "layout": data,
            });

            let name = path
                .file_name()
                .expect("file name")
                .to_string_lossy()
                .strip_suffix(".html")
                .expect("strip .html")
                .to_string();

            layouts.insert(name, template.render(&globals)?);
        }

        Ok(LayoutHandler { layouts })
    }

    pub fn get_layout(&self, name: &str) -> Option<&String> {
        self.layouts.get(name)
    }
}
