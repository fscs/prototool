use anyhow::{bail, Context, Result};
use askama::Template;
use chrono::{NaiveDateTime, Local};

use std::fs;
use std::path::{Path, PathBuf};

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

pub fn create_post(root: &Path, lang: &str, target: &str) -> Result<PathBuf> {
    let content_dir = find_content_dir(root, lang);

    if !content_dir.exists() {
        bail!("content dir doesnt exist yet")
    }
    
    let target_path = content_dir.join(target);

    let Some(category_path) = target_path.parent() else {
        bail!("unable to determine category path")
    };

    fs::create_dir_all(category_path).context("unable to create category path")?;
    
    if target_path.exists() {
        bail!("target path already exists");
    }

    fs::write(&target_path, "").context("unable to create file")?;

    Ok(target_path)
}

pub fn edit(path: &Path, editor: &str) -> Result<()> {
    // Spawn editor process
    let mut cmd = std::process::Command::new(editor)
        .arg(path)
        .spawn()
        .context("editor {editor} not found")?;

    cmd.wait()?;

    Ok(())
}

pub fn write_post_template(path: &Path, date: NaiveDateTime) -> Result<()> {
    let date_formatted = date.format("%Y-%m-%dT%H:%M:%S");

    let template = PostTemplate {
        date_machine: date_formatted.to_string(),
    };

    let result = template.render().context("failed to render post template")?;

    fs::write(path, result).context("failed to write post template")?;

    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use askama::Template;
    use pretty_assertions::assert_eq;
    use tempfile::tempdir;
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    
    use std::fs;
    
    use super::PostTemplate;

    #[test]
    fn content_dir_doesnt_exist() {
        let tmpdir = tempdir().unwrap();

        let result = super::create_post(tmpdir.path(), "de", "news/test.md");

        assert!(result.is_err())
    }

    #[test]
    fn category_doesnt_exist() {
        let tmpdir = tempdir().unwrap();
        let content_dir = tmpdir.path().join("content/de/");
        
        fs::create_dir_all(&content_dir).unwrap();

        let result = super::create_post(tmpdir.path(), "de", "news/test.md").unwrap();
        
        let expected = content_dir.join("news/test.md");
        assert_eq!(result, expected)
    }
    
    static POST: &'static str = include_str!("../tests/post.md");

    #[test]
    fn post_template() {
        let template = PostTemplate {
            date_machine: "2022-05-27T07:30:15".to_string(),
        };

        assert_eq!(template.render().unwrap(), POST);
    }

    #[test]
    fn write_post_template() {
        let tmpfile = tempfile::NamedTempFile::new().unwrap();
        let time = NaiveTime::from_hms_opt(7, 30, 15).unwrap();
        let date = NaiveDate::from_ymd_opt(2022, 5, 27).unwrap();
        let datetime = NaiveDateTime::new(date, time);

        super::write_post_template(tmpfile.path(), datetime).unwrap();

        assert_eq!(fs::read_to_string(tmpfile).unwrap(), POST);
    }
}
