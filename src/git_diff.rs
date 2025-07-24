use git2::{DiffOptions, ObjectType, Repository, BranchType};
use similar::TextDiff;
use std::rc::Rc;
use std::cell::RefCell;

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
