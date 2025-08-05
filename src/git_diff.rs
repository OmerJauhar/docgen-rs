use git2::{DiffOptions, ObjectType, Repository, BranchType};
use similar::TextDiff;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct GitDiffData {
    pub files: Vec<GitFileChange>,
    pub total_additions: usize,
    pub total_deletions: usize,
    pub total_files: usize,
}

#[derive(Clone, Debug)]
pub struct GitFileChange {
    pub file_path: String,
    pub status: String, // "modified", "added", "deleted"
    pub diff_lines: Vec<String>,
    pub additions: usize,
    pub deletions: usize,
}

impl GitDiffData {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            total_additions: 0,
            total_deletions: 0,
            total_files: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    pub fn summary(&self) -> String {
        format!("{} files changed, {} insertions(+), {} deletions(-)", 
                self.total_files, self.total_additions, self.total_deletions)
    }
}

pub fn get_working_directory_diff(repo_path: &str) -> Result<GitDiffData, git2::Error> {
    let repo = Repository::open(repo_path)?;
    let head = repo.head()?.peel_to_commit()?;
    let head_tree = head.tree()?;
    
    let mut diff_opts = DiffOptions::new();
    diff_opts.include_untracked(true);
    diff_opts.include_ignored(false);
    
    let diff = repo.diff_tree_to_workdir_with_index(Some(&head_tree), Some(&mut diff_opts))?;
    
    // Use Rc<RefCell<>> to handle borrowing issues
    let file_changes = Rc::new(RefCell::new(Vec::<GitFileChange>::new()));
    let current_file_index = Rc::new(RefCell::new(None::<usize>));
    let stats = Rc::new(RefCell::new((0usize, 0usize, 0usize))); // (total_files, total_additions, total_deletions)
    
    // Clone for closures
    let file_changes_1 = Rc::clone(&file_changes);
    let current_file_index_1 = Rc::clone(&current_file_index);
    let stats_1 = Rc::clone(&stats);
    
    let file_changes_2 = Rc::clone(&file_changes);
    let current_file_index_2 = Rc::clone(&current_file_index);
    let stats_2 = Rc::clone(&stats);
    
    // Collect all diff information
    diff.foreach(
        &mut move |delta, _progress| {
            let file_path = delta.new_file().path()
                .or_else(|| delta.old_file().path())
                .unwrap_or_else(|| std::path::Path::new("unknown"))
                .display()
                .to_string();
            
            let status = match delta.status() {
                git2::Delta::Added => "added",
                git2::Delta::Deleted => "deleted", 
                git2::Delta::Modified => "modified",
                git2::Delta::Renamed => "renamed",
                git2::Delta::Copied => "copied",
                git2::Delta::Untracked => "untracked",
                _ => "unknown",
            };

            // Create a new file entry
            file_changes_1.borrow_mut().push(GitFileChange {
                file_path,
                status: status.to_string(),
                diff_lines: Vec::new(),
                additions: 0,
                deletions: 0,
            });

            let new_index = file_changes_1.borrow().len() - 1;
            *current_file_index_1.borrow_mut() = Some(new_index);
            stats_1.borrow_mut().0 += 1; // total_files
            true
        },
        None,
        Some(&mut |_delta, _binary| true),
        Some(&mut move |_delta, _hunk, line| {
            if let Some(idx) = *current_file_index_2.borrow() {
                if let Some(last_file) = file_changes_2.borrow_mut().get_mut(idx) {
                    let content = String::from_utf8_lossy(line.content()).trim_end().to_string();
                    match line.origin() {
                        '+' => {
                            last_file.diff_lines.push(format!("+{}", content));
                            last_file.additions += 1;
                            stats_2.borrow_mut().1 += 1; // total_additions
                        },
                        '-' => {
                            last_file.diff_lines.push(format!("-{}", content));
                            last_file.deletions += 1;
                            stats_2.borrow_mut().2 += 1; // total_deletions
                        },
                        ' ' => {
                            last_file.diff_lines.push(format!(" {}", content));
                        },
                        _ => {
                            // Handle other line types (like hunk headers)
                            last_file.diff_lines.push(format!(" {}", content));
                        }
                    }
                }
            }
            true
        }),
    )?;

    let files = Rc::try_unwrap(file_changes).unwrap().into_inner();
    let (total_files, total_additions, total_deletions) = Rc::try_unwrap(stats).unwrap().into_inner();
    
    Ok(GitDiffData {
        files,
        total_additions,
        total_deletions,
        total_files,
    })
}

pub fn diff_branch(
    path: &str,
    branch: &str,
) -> Result<Vec<(String, Vec<String>)>, git2::Error> {
    let repo = Repository::open(path)?;
    let head_commit = repo.head()?.peel(ObjectType::Commit)?.into_commit().unwrap();
    let branch_ref = repo.find_branch(branch, BranchType::Local)?;
    let branch_commit = branch_ref.into_reference().peel_to_commit()?;

    let mut diff_opts = DiffOptions::new();
    let diff = repo.diff_tree_to_tree(
        Some(&head_commit.tree()?),
        Some(&branch_commit.tree()?),
        Some(&mut diff_opts),
    )?;
    let file_paths = Rc::new(RefCell::new(Vec::new()));
    let hunk_headers = Rc::new(RefCell::new(Vec::new()));
    let current_file_idx = Rc::new(RefCell::new(None));

    // Separate clones for each closure
    let file_paths_file_cl = Rc::clone(&file_paths);
    let hunk_headers_file_cl = Rc::clone(&hunk_headers);
    let current_file_idx_file_cl = Rc::clone(&current_file_idx);

    let hunk_headers_hunk_cl = Rc::clone(&hunk_headers);
    let current_file_idx_hunk_cl = Rc::clone(&current_file_idx);

    diff.foreach(
        &mut move |delta, _| {
            let path = delta.new_file().path().unwrap().display().to_string();
            file_paths_file_cl.borrow_mut().push(path);
            hunk_headers_file_cl.borrow_mut().push(Vec::new());
            *current_file_idx_file_cl.borrow_mut() = Some(file_paths_file_cl.borrow().len() - 1);
            true
        },
        None,
        None,
        Some(&mut move |_, hunk, _| {
            if let (Some(hunk), Some(idx)) = (hunk, *current_file_idx_hunk_cl.borrow()) {
                hunk_headers_hunk_cl.borrow_mut()[idx].push(format!(
                    "@@ -{},{} +{},{} @@",
                    hunk.old_start(),
                    hunk.old_lines(),
                    hunk.new_start(),
                    hunk.new_lines()
                ));
            }
            true
        }),
    )?;

    // Load file contents and build diff for each file
    let mut results = Vec::new();
    let head_tree = head_commit.tree()?;
    let branch_tree = branch_commit.tree()?;
    for (file, _hunks) in file_paths.borrow().iter().cloned().zip(hunk_headers.borrow().iter().cloned()) {
        let old_blob = repo.find_blob(head_tree.get_path(&std::path::Path::new(&file))?.id())?;
        let new_blob = repo.find_blob(branch_tree.get_path(&std::path::Path::new(&file))?.id())?;
        let old = old_blob.content();
        let new = new_blob.content();
        let old_str = String::from_utf8_lossy(old);
        let new_str = String::from_utf8_lossy(new);
        let diff = TextDiff::from_lines(&old_str, &new_str);

        let mut lines = Vec::new();
        for change in diff.iter_all_changes() {
            let sign = match change.tag() {
                similar::ChangeTag::Delete => "-",
                similar::ChangeTag::Insert => "+",
                similar::ChangeTag::Equal => " ",
            };
            lines.push(format!("{}{}", sign, change));
        }
        results.push((file, lines));
    }
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_diff_branch() {
        let repo_path = "/home/meow/repos/Spaghetti-Pinns-Website";
        let branch = "main"; // Change if your main branch is named differently
        match diff_branch(repo_path, branch) {
            Ok(results) => {
                for (file, lines) in results {
                    println!("File: {}", file);
                    for line in lines {
                        println!("{}", line);
                    }
                }
            },
            Err(e) => println!("Error: {}", e),
        }
    }
}
