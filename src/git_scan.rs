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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_list_local_branches() {
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
}
