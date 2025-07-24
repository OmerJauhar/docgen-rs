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