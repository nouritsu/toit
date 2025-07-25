use crate::layouts::LayoutHandler;
use serde::Deserialize;
use thiserror::Error;

pub struct Post {
    pub title: String,
    pub subtext: String,
    pub layout: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct PostFrontMatter {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub subtext: String,
    pub layout: String,
}

#[derive(Error, Debug)]
pub enum HTMLGenError {
    #[error("layout {0} not found")]
    InvalidLayout(String),

    #[error("failed to process liquid template: {0}")]
    LiquidError(#[from] liquid::Error),
}

impl Post {
    pub fn from_parts(front_matter: PostFrontMatter, content: String) -> Self {
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
            .ok_or(HTMLGenError::InvalidLayout(self.layout.clone()))?;

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
