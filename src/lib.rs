use std::path::{Path, PathBuf};
use std::{fmt, fs, io};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    InvalidIndex(usize),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "{}", e),
            Error::InvalidIndex(i) => write!(f, "invalid index: {}", i),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

/// 현재 디렉토리의 서브디렉토리 목록을 대소문자 무시 알파벳순으로 반환 (숨김 포함)
pub fn list_subdirs(path: &Path) -> Result<Vec<String>, Error> {
    let mut dirs: Vec<String> = fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let file_type = entry.file_type().ok()?;
            if file_type.is_dir() || (file_type.is_symlink() && entry.path().is_dir()) {
                entry.file_name().into_string().ok()
            } else {
                None
            }
        })
        .collect();

    dirs.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    Ok(dirs)
}

/// 0-based 인덱스로 서브디렉토리 경로를 반환
pub fn get_subdir(path: &Path, index: usize) -> Result<PathBuf, Error> {
    let dirs = list_subdirs(path)?;
    if index >= dirs.len() {
        return Err(Error::InvalidIndex(index));
    }
    Ok(path.join(&dirs[index]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn setup() -> TempDir {
        let tmp = TempDir::new().unwrap();
        let base = tmp.path();
        fs::create_dir(base.join("banana")).unwrap();
        fs::create_dir(base.join("Apple")).unwrap();
        fs::create_dir(base.join(".hidden")).unwrap();
        fs::create_dir(base.join("cherry")).unwrap();
        fs::File::create(base.join("file.txt")).unwrap();
        tmp
    }

    #[test]
    fn test_list_sorted() {
        let tmp = setup();
        let dirs = list_subdirs(tmp.path()).unwrap();
        assert_eq!(dirs, vec![".hidden", "Apple", "banana", "cherry"]);
    }

    #[test]
    fn test_list_excludes_files() {
        let tmp = setup();
        let dirs = list_subdirs(tmp.path()).unwrap();
        assert!(!dirs.contains(&"file.txt".to_string()));
    }

    #[test]
    fn test_list_includes_hidden() {
        let tmp = setup();
        let dirs = list_subdirs(tmp.path()).unwrap();
        assert!(dirs.contains(&".hidden".to_string()));
    }

    #[test]
    fn test_list_empty_dir() {
        let tmp = TempDir::new().unwrap();
        let dirs = list_subdirs(tmp.path()).unwrap();
        assert!(dirs.is_empty());
    }

    #[test]
    fn test_get_subdir_valid() {
        let tmp = setup();
        let dirs = list_subdirs(tmp.path()).unwrap();
        let target = get_subdir(tmp.path(), 0).unwrap();
        assert_eq!(target, tmp.path().join(&dirs[0]));
    }

    #[test]
    fn test_get_subdir_out_of_range() {
        let tmp = setup();
        let dirs = list_subdirs(tmp.path()).unwrap();
        assert!(get_subdir(tmp.path(), dirs.len()).is_err());
    }
}
