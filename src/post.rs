use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use askama::Template;
use chrono::NaiveDateTime;

#[derive(Template)]
#[template(path = "../templates/post.md")]
pub struct PostTemplate {
    date_machine: String,
}

fn find_content_dir(root: &Path, lang: &str) -> PathBuf {
    let mut result = root.to_owned();
    result.push("content");
    result.push(lang);

    result
}

pub fn create_post(
    content: &str,
    root: &Path,
    lang: &str,
    target: &str,
    force: bool,
) -> Result<PathBuf> {
    let content_dir = find_content_dir(root, lang);

    if !content_dir.exists() {
        bail!("content dir doesnt exist yet")
    }

    let target_path = content_dir.join(target);

    let Some(category_path) = target_path.parent() else {
        bail!("unable to determine category path")
    };

    fs::create_dir_all(category_path).context("unable to create category path")?;

    if target_path.exists() && !force {
        bail!("target path already exists");
    }

    fs::write(&target_path, content).context("unable to create file")?;

    Ok(target_path)
}

pub fn edit(path: &Path) -> Result<()> {
    opener::open(path).context("unable to open file")
}

pub fn render_post_template(date: &NaiveDateTime) -> Result<String> {
    let date_formatted = date.format("%Y-%m-%dT%H:%M:%S");

    let template = PostTemplate {
        date_machine: date_formatted.to_string(),
    };

    return template.render().context("failed to render post template");
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use askama::Template;
    use chrono::NaiveDate;
    use pretty_assertions::assert_eq;
    use tempfile::tempdir;

    use std::fs;

    use super::PostTemplate;

    #[test]
    fn content_dir_doesnt_exist() {
        let tmpdir = tempdir().unwrap();

        let result = super::create_post("", tmpdir.path(), "de", "news/test.md", false);

        assert!(result.is_err())
    }

    #[test]
    fn category_doesnt_exist() {
        let tmpdir = tempdir().unwrap();
        let content_dir = tmpdir.path().join("content/de/");

        fs::create_dir_all(&content_dir).unwrap();

        let result = super::create_post("", tmpdir.path(), "de", "news/test.md", false).unwrap();

        let expected = content_dir.join("news/test.md");
        assert_eq!(result, expected)
    }

    static POST: &str = include_str!("../tests/post.md");

    #[test]
    fn post_template() {
        let template = PostTemplate {
            date_machine: "2022-05-27T07:30:15".to_string(),
        };

        assert_eq!(template.render().unwrap(), POST);
    }

    #[test]
    fn render_post_template() {
        let datetime = NaiveDate::from_ymd_opt(2022, 5, 27)
            .unwrap()
            .and_hms_opt(7, 30, 15)
            .unwrap();

        let result = super::render_post_template(&datetime).unwrap();

        assert_eq!(result, POST);
    }
}
