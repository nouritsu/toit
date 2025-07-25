use clap::Parser;
use headless_chrome::{Browser, LaunchOptionsBuilder};
use std::{fs, path::PathBuf};
use tempdir::TempDir;
use toit::{
    layouts::LayoutHandler,
    post::{Post, PostFrontMatter},
    render::render_html,
};

#[derive(Parser)]
struct Args {
    /// Path to root directory
    #[clap(short, long = "root", default_value = ".")]
    root: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let lh = LayoutHandler::build(&args.root.join("layouts"))?;

    let mut post_files = Vec::new();
    let posts_html_dir = TempDir::new("posts_html")?;

    for post in fs::read_dir(&args.root.join("posts"))? {
        let path = post?.path();
        let md = markdown_parser::read_file(&path)?;

        let fm: PostFrontMatter = toml::from_str(md.front_matter())?;
        let post = Post::from_parts(fm, md.content().to_owned());

        let html = post.to_html(&lh)?;
        let tmp_file = posts_html_dir
            .path()
            .join(path.with_extension("html").file_name().expect("file name"));
        fs::write(&tmp_file, html)?;
        post_files.push(tmp_file);
    }

    let output = args.root.join("output");
    fs::create_dir_all(&output)?;

    let browser = Browser::new(
        LaunchOptionsBuilder::default()
            .window_size(Some((1200, 1200)))
            .build()?,
    )?;

    for post in post_files {
        let data = render_html(&browser, &post);
        let png_file = output.join(post.with_extension("png").file_name().expect("file name"));
        fs::write(png_file, data?)?;
    }

    Ok(())
}
