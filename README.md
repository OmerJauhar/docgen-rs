# docgen-rs

A modern, TUI-based documentation assistant for Rust projects, powered by [Ratatui](https://github.com/ratatui-org/ratatui). docgen-rs helps developers visualize, analyze, and document code changes efficiently, with a professional and intuitive terminal interface.

## Features

- **📊 Module Impact Heatmap:**
  - Visualize changed files/modules from Git diffs.
  - See lines added/removed, functions modified, and impact bars color-coded by change weight.

- **⚙️ Code Change Flow Diagram:**
  - Step-by-step horizontal diagram: [Git Diff] → [Metadata Form] → [LLM Analysis] → [Markdown Gen] → [PDF Export].
  - Highlights the current step and animates transitions.

- **💸 LLM Token Cost Estimator:**
  - Estimates token usage for diffs and form answers.
  - Color-coded gauge (green/yellow/red) for cost awareness.

- **📝 Metadata Form:**
  - Guided form for describing code changes, DB changes, extensibility, and performance notes.

- **Professional TUI Design:**
  - Clean, bordered panels with headings and keyboard navigation.
  - Tabs for switching between Form, Git Diff Viewer, and LLM Stats.

## Screenshots

> _Add screenshots here after running the app!_

## Usage

### Run the TUI
```sh
cargo run
```

### Keyboard Navigation
- **←/→**: Switch between Form, Git Diff Viewer, and LLM Stats
- **↑/↓**: Navigate form steps (when in Form view)
- **Esc**: Quit

## Development

### Prerequisites
- Rust (edition 2021 or later)
- [Ratatui](https://github.com/ratatui-org/ratatui) crate (see `Cargo.toml`)
- [crossterm](https://github.com/crossterm-rs/crossterm) for terminal handling

### Project Structure
- `src/tui/`
  - `app.rs`: Application state and enums
  - `layout.rs`: Layout logic for the TUI
  - `widgets.rs`: All custom widgets and panels
  - `events.rs`: Keyboard and input handling
- `src/main.rs`: Entry point and terminal setup

### Customization
- The TUI is modular and ready for expansion:
  - Connect real Git diffs and LLM APIs
  - Add more panels or export options
  - Refine the UI with more Ratatui widgets

## Git Utilities

### `diff_branch`
```rust
// src/git_diff.rs
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
```

### `list_local_branches`
```rust
// src/git_scan.rs
use git2::{Repository, BranchType};

pub fn list_local_branches(path: &str) -> Result<Vec<String>, git2::Error> {
    let repo = Repository::open(path)?;
    let branches = repo.branches(Some(BranchType::Local))?;
    let mut names = Vec::new();
    for branch in branches {
        let (b, _) = branch?;
        if let Some(name) = b.name()? {
            names.push(name.to_string());
        }
    }
    Ok(names)
}
```

## Testing

Integration tests for these utilities are provided in `tests/git_tests.rs`:

```rust
use docgen_rs::git_diff::diff_branch;
use docgen_rs::git_scan::list_local_branches;

#[test]
fn test_diff_branch_integration() {
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

#[test]
fn test_list_local_branches_integration() {
    let repo_path = "/home/meow/repos/Spaghetti-Pinns-Website";
    match list_local_branches(repo_path) {
        Ok(branches) => {
            println!("Branches:");
            for branch in branches {
                println!("{}", branch);
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}
```

