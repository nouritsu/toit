use crate::layouts::LayoutHandler;
use serde::Deserialize;
use thiserror::Error;

pub struct Post<'md> {
    pub title: &'md str,
    pub subtext: &'md str,
    pub layout: &'md str,
    pub content: &'md str,
}

#[derive(Deserialize)]
pub struct PostFrontMatter<'md> {
    #[serde(default)]
    pub title: &'md str,
    #[serde(default)]
    pub subtext: &'md str,
    pub layout: &'md str,
}

#[derive(Error, Debug)]
pub enum HTMLGenError {
    #[error("layout {0} not found")]
    InvalidLayout(String),

    #[error("failed to process liquid template: {0}")]
    LiquidError(#[from] liquid::Error),
}

impl<'md> Post<'md> {
    pub fn from_parts(front_matter: PostFrontMatter<'md>, content: &'md str) -> Self {
        Post {
            title: front_matter.title,
            subtext: front_matter.subtext,
            layout: front_matter.layout,
            content,
        }
    }

    pub fn to_html(&self, lh: &LayoutHandler) -> Result<String, HTMLGenError> {
        let layout = lh
            .get_layout(&self.layout)
            .ok_or(HTMLGenError::InvalidLayout(self.layout.to_owned()))?;

        let template = liquid::ParserBuilder::with_stdlib()
            .build()?
            .parse(layout)?;

        let globals = liquid::object!({
            "title": self.title,
            "subtext": self.subtext,
            "content": self.content,
        });

        Ok(template.render(&globals)?)
    }
}
