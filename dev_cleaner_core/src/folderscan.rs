use ahash::RandomState;
use hashbrown::HashSet;
use std::{
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub fn scan_folders(path: impl AsRef<Path>, target_dirs: &[&str]) -> Vec<PathBuf> {
    let name_matches_target = |p: &Path| {
        p.file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| target_dirs.contains(&n))
    };

    let skip_dirs: HashSet<PathBuf, RandomState> = HashSet::with_hasher(RandomState::new());

    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .scan(skip_dirs, |skip_dirs, entry| {
            let entry_path = entry.path().to_path_buf();

            if skip_dirs.iter().any(|skip| entry_path.starts_with(skip)) {
                return Some(None);
            }

            if entry.file_type().is_dir() && name_matches_target(&entry_path) {
                skip_dirs.insert(entry_path.clone());
                Some(Some(entry_path))
            } else {
                Some(None)
            }
        })
        .flatten()
        .collect()
}

pub fn remove_folders(paths: Vec<PathBuf>) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let fold_into_tuple = |(mut deleted, mut errors): (Vec<PathBuf>, Vec<PathBuf>),
                           (original_path, result_of_removal)| {
        match result_of_removal {
            Ok(_) => {
                deleted.push(original_path);
            }
            Err(e) => {
                eprintln!("{e}");
                errors.push(original_path);
            }
        }
        (deleted, errors)
    };

    let (deleted, errored) = paths
        .into_iter()
        .map(|p| (p.clone(), fs::remove_dir_all(p)))
        .fold((Vec::new(), Vec::new()), fold_into_tuple);

    (deleted, errored)
}
