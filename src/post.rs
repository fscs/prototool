use anyhow::{bail, Context, Result};

use std::fs;
use std::path::{Path, PathBuf};

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

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::fs;
    use tempfile::tempdir;

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
}
