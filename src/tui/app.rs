use crate::git_diff::GitDiffData;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MainView {
    BranchSelection,
    Form,
    GitDiff,
    LLMStats,
    UserInfo,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UserInfoField {
    Name,
    EmployeeNumber,
    Designation,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub name: String,
    pub employee_number: String,
    pub designation: String,
    pub is_configured: bool,
}

impl UserInfo {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            employee_number: String::new(),
            designation: String::new(),
            is_configured: false,
        }
    }

    pub fn is_complete(&self) -> bool {
        !self.name.trim().is_empty() 
            && !self.employee_number.trim().is_empty() 
            && !self.designation.trim().is_empty()
    }

    pub fn mark_configured(&mut self) {
        if self.is_complete() {
            self.is_configured = true;
        }
    }

    pub fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir()
            .ok_or("Could not find config directory")?
            .join("docgen-rs");
        
        fs::create_dir_all(&config_dir)?;
        let config_file = config_dir.join("user_info.json");
        let json = serde_json::to_string_pretty(self)?;
        fs::write(config_file, json)?;
        Ok(())
    }

    pub fn load_from_file() -> Self {
        let config_dir = match dirs::config_dir() {
            Some(dir) => dir.join("docgen-rs"),
            None => return UserInfo::new(),
        };
        
        let config_file = config_dir.join("user_info.json");
        
        if config_file.exists() {
            match fs::read_to_string(&config_file) {
                Ok(content) => {
                    match serde_json::from_str::<UserInfo>(&content) {
                        Ok(user_info) => user_info,
                        Err(_) => UserInfo::new(),
                    }
                }
                Err(_) => UserInfo::new(),
            }
        } else {
            UserInfo::new()
        }
    }
}

#[derive(Clone)]
pub struct FileImpact {
    pub filename: String,
    pub lines_added: usize,
    pub lines_removed: usize,
    pub functions_modified: usize,
    pub impact_score: usize, // for color/bar
}

#[derive(Clone)]
pub struct TokenStats {
    pub diff_tokens: usize,
    pub form_tokens: usize,
    pub total_tokens: usize,
    pub estimated_cost_usd: f64, // Cost in USD
    pub estimated_cost_pkr: f64, // Cost in PKR
}

impl TokenStats {
    pub fn new() -> Self {
        Self {
            diff_tokens: 0,
            form_tokens: 0,
            total_tokens: 0,
            estimated_cost_usd: 0.0,
            estimated_cost_pkr: 0.0,
        }
    }

    pub fn calculate_cost(&mut self) {
        // Using GPT-4 pricing as reference: ~$0.03 per 1K tokens for input
        const COST_PER_1K_TOKENS: f64 = 0.03;
        const USD_TO_PKR_RATE: f64 = 280.0;
        
        self.estimated_cost_usd = (self.total_tokens as f64 / 1000.0) * COST_PER_1K_TOKENS;
        self.estimated_cost_pkr = self.estimated_cost_usd * USD_TO_PKR_RATE;
    }
}

// Token estimation utility
pub fn estimate_tokens(text: &str) -> usize {
    // Rough estimation: 1 token ≈ 4 characters for English text
    // This is a simplified approximation, real tokenizers are more complex
    let char_count = text.len();
    let word_count = text.split_whitespace().count();
    
    // More accurate estimation considering:
    // - Average word length
    // - Punctuation and special characters
    // - Code tokens tend to be shorter
    let estimated_tokens = if char_count == 0 {
        0
    } else {
        // Use a hybrid approach: base on character count but adjust for word boundaries
        let base_tokens = char_count / 4;
        let word_adjustment = word_count / 3; // Words typically don't align perfectly with 4-char tokens
        std::cmp::max(base_tokens, word_adjustment)
    };
    
    estimated_tokens
}

// Estimate function modifications from diff lines
pub fn estimate_function_changes(diff_lines: &[String]) -> usize {
    let mut function_count = 0;
    
    for line in diff_lines {
        if line.starts_with('+') || line.starts_with('-') {
            let content = &line[1..]; // Remove +/- prefix
            
            // Look for function patterns in various languages
            if content.trim_start().contains("fn ") ||           // Rust
               content.trim_start().contains("function ") ||     // JavaScript
               content.trim_start().contains("def ") ||          // Python
               content.trim_start().contains("void ") ||         // C/C++
               content.trim_start().contains("int ") ||          // C/C++
               content.trim_start().contains("public ") ||       // Java/C#
               content.trim_start().contains("private ") ||      // Java/C#
               content.trim_start().contains("async ") ||        // Modern languages
               content.contains("=>") ||                         // Arrow functions
               content.contains("func ") {                       // Go, Swift
                function_count += 1;
            }
        }
    }
    
    function_count
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FormStep {
    ProblemStatement,
    HighLevelOverview,
    CodeStructure,
    KeyChanges,
    FutureConsiderations,
}

pub struct App {
    pub current_step: FormStep,
    pub input_buffers: [String; 5],
    pub main_view: MainView,
    pub file_impacts: Vec<FileImpact>,
    pub token_stats: TokenStats,
    pub flow_step: usize, // 0..=4 for diagram
    pub current_branch: String,
    pub git_status: String,
    pub project_folder: String, // New field for selected folder
    pub available_branches: Vec<String>, // List of available branches
    pub selected_branch_index: usize, // Currently selected branch in the list
    pub git_diff_data: GitDiffData, // Store git diff data
    pub can_submit: bool, // Whether form can be submitted
    pub submit_attempted: bool, // Whether user has tried to submit
    pub user_info: UserInfo, // User information
    pub current_user_field: UserInfoField, // Currently selected user info field
    pub user_info_buffer: String, // Buffer for editing user info
    pub show_user_info_prompt: bool, // Whether to show setup prompt
}

impl App {
    pub fn new() -> Self {
        let user_info = UserInfo::load_from_file();
        let show_prompt = !user_info.is_configured;
        
        let mut app = Self {
            current_step: FormStep::ProblemStatement,
            input_buffers: Default::default(),
            main_view: MainView::BranchSelection,
            file_impacts: vec![], // Will be populated from git diff data
            token_stats: TokenStats::new(),
            flow_step: 0,
            current_branch: "main".to_string(),
            git_status: "3 files changed, 18 insertions(+), 4 deletions(-)".to_string(),
            project_folder: ".".to_string(), // Default to current dir
            available_branches: vec![], // Will be populated when project is selected
            selected_branch_index: 0,
            git_diff_data: GitDiffData::new(),
            can_submit: false,
            submit_attempted: false,
            user_info,
            current_user_field: UserInfoField::Name,
            user_info_buffer: String::new(),
            show_user_info_prompt: show_prompt,
        };

        // Try to load branches from current directory on startup
        if let Ok(_) = app.load_branches(".") {
            app.git_status = format!("Loaded {} branches from current directory", app.available_branches.len());
            
            // Also try to load git diff automatically
            if let Ok(_) = app.load_git_diff() {
                app.git_status = format!("{} | {}", app.git_status, app.git_diff_data.summary());
            } else {
                // Even if no git changes, update token counts for form
                app.update_token_counts();
            }
        } else {
            app.git_status = "Not a git repository or no branches found".to_string();
            // Update token counts even without git data
            app.update_token_counts();
        }

        app
    }

    pub fn load_branches(&mut self, path: &str) -> Result<(), git2::Error> {
        use crate::git_scan::list_local_branches;
        self.available_branches = list_local_branches(path)?;
        self.selected_branch_index = 0;
        // Set current branch to the first available branch if any
        if !self.available_branches.is_empty() {
            self.current_branch = self.available_branches[0].clone();
        }
        Ok(())
    }

    pub fn select_current_branch(&mut self) {
        if !self.available_branches.is_empty() && self.selected_branch_index < self.available_branches.len() {
            self.current_branch = self.available_branches[self.selected_branch_index].clone();
        }
    }

    pub fn load_git_diff(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        use crate::git_diff::get_working_directory_diff;
        
        self.git_diff_data = get_working_directory_diff(&self.project_folder)?;
        self.git_status = self.git_diff_data.summary();
        
        // Update file impacts from git diff data
        self.update_file_impacts();
        
        // Update token counts
        self.update_token_counts();
        
        self.validate_form();
        Ok(())
    }

    pub fn update_file_impacts(&mut self) {
        self.file_impacts.clear();
        
        for file_change in &self.git_diff_data.files {
            // Calculate impact score based on changes
            let change_magnitude = file_change.additions + file_change.deletions;
            let impact_score = std::cmp::min(change_magnitude, 100); // Cap at 100
            
            // Estimate function modifications (rough heuristic)
            let functions_modified = estimate_function_changes(&file_change.diff_lines);
            
            self.file_impacts.push(FileImpact {
                filename: file_change.file_path.clone(),
                lines_added: file_change.additions,
                lines_removed: file_change.deletions,
                functions_modified,
                impact_score,
            });
        }
    }

    pub fn update_token_counts(&mut self) {
        // Calculate form tokens
        let mut form_text = String::new();
        for buffer in &self.input_buffers {
            form_text.push_str(buffer);
            form_text.push(' ');
        }
        self.token_stats.form_tokens = estimate_tokens(&form_text);
        
        // Calculate diff tokens
        let mut diff_text = String::new();
        for file_change in &self.git_diff_data.files {
            diff_text.push_str(&format!("File: {}\n", file_change.file_path));
            for line in &file_change.diff_lines {
                diff_text.push_str(line);
                diff_text.push('\n');
            }
        }
        self.token_stats.diff_tokens = estimate_tokens(&diff_text);
        
        // Update total and cost
        self.token_stats.total_tokens = self.token_stats.form_tokens + self.token_stats.diff_tokens;
        self.token_stats.calculate_cost();
    }

    pub fn validate_form(&mut self) {
        // Update token counts every time validation runs
        self.update_token_counts();
        
        // Check if all form fields are filled
        let all_fields_filled = self.input_buffers.iter().all(|buffer| !buffer.trim().is_empty());
        
        // Check if git diff has changes
        let has_git_changes = !self.git_diff_data.is_empty();
        
        // Update can_submit status
        self.can_submit = all_fields_filled && has_git_changes;
    }

    pub fn get_form_errors(&self) -> Vec<String> {
        let mut errors = Vec::new();
        
        if self.submit_attempted {
            // Check each form field
            let question_names = [
                "Problem Statement",
                "High-Level Overview", 
                "Code Structure",
                "Key Changes",
                "Future Considerations"
            ];
            
            for (i, buffer) in self.input_buffers.iter().enumerate() {
                if buffer.trim().is_empty() {
                    errors.push(format!("{} is required", question_names[i]));
                }
            }
            
            // Check git diff
            if self.git_diff_data.is_empty() {
                errors.push("No git changes found. Make some changes to your code first.".to_string());
            }
        }
        
        errors
    }

    pub fn attempt_submit(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        self.submit_attempted = true;
        self.validate_form();
        
        if self.can_submit {
            self.submit_to_llm()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn submit_to_llm(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Prepare the prompt with form data and git diff
        let mut prompt = String::new();
        prompt.push_str("# Code Documentation Request\n\n");
        
        // Add form data
        let question_names = [
            "Problem Statement",
            "High-Level Overview", 
            "Code Structure",
            "Key Changes",
            "Future Considerations"
        ];
        
        for (i, buffer) in self.input_buffers.iter().enumerate() {
            prompt.push_str(&format!("## {}\n{}\n\n", question_names[i], buffer));
        }
        
        // Add git diff data
        prompt.push_str("## Git Changes\n");
        prompt.push_str(&format!("Branch: {}\n", self.current_branch));
        prompt.push_str(&format!("Summary: {}\n\n", self.git_diff_data.summary()));
        
        for file_change in &self.git_diff_data.files {
            prompt.push_str(&format!("### {} ({})\n", file_change.file_path, file_change.status));
            prompt.push_str("```diff\n");
            for line in &file_change.diff_lines {
                prompt.push_str(&format!("{}\n", line));
            }
            prompt.push_str("```\n\n");
        }
        
        // TODO: Send to LLM API
        // For now, just update status
        self.git_status = "Submitted to LLM successfully!".to_string();
        self.flow_step = 4; // Move to final step
        
        Ok(())
    }

    pub fn enter_user_info_mode(&mut self) {
        self.main_view = MainView::UserInfo;
        self.current_user_field = UserInfoField::Name;
        // Only set buffer if it's empty - don't overwrite existing content
        if self.user_info_buffer.is_empty() {
            self.user_info_buffer = self.user_info.name.clone();
        }
    }

    pub fn next_user_info_field(&mut self) {
        // Save current buffer to user info
        self.save_current_user_field();
        
        // Move to next field
        self.current_user_field = match self.current_user_field {
            UserInfoField::Name => UserInfoField::EmployeeNumber,
            UserInfoField::EmployeeNumber => UserInfoField::Designation,
            UserInfoField::Designation => UserInfoField::Name,
        };
        
        // Load new field into buffer
        self.user_info_buffer = match self.current_user_field {
            UserInfoField::Name => self.user_info.name.clone(),
            UserInfoField::EmployeeNumber => self.user_info.employee_number.clone(),
            UserInfoField::Designation => self.user_info.designation.clone(),
        };
    }

    pub fn previous_user_info_field(&mut self) {
        // Save current buffer to user info
        self.save_current_user_field();
        
        // Move to previous field
        self.current_user_field = match self.current_user_field {
            UserInfoField::Name => UserInfoField::Designation,
            UserInfoField::EmployeeNumber => UserInfoField::Name,
            UserInfoField::Designation => UserInfoField::EmployeeNumber,
        };
        
        // Load new field into buffer
        self.user_info_buffer = match self.current_user_field {
            UserInfoField::Name => self.user_info.name.clone(),
            UserInfoField::EmployeeNumber => self.user_info.employee_number.clone(),
            UserInfoField::Designation => self.user_info.designation.clone(),
        };
    }

    pub fn save_current_user_field(&mut self) {
        match self.current_user_field {
            UserInfoField::Name => self.user_info.name = self.user_info_buffer.clone(),
            UserInfoField::EmployeeNumber => self.user_info.employee_number = self.user_info_buffer.clone(),
            UserInfoField::Designation => self.user_info.designation = self.user_info_buffer.clone(),
        }
    }

    pub fn save_user_info(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Save current buffer first
        self.save_current_user_field();
        
        if self.user_info.is_complete() {
            self.user_info.mark_configured();
            self.user_info.save_to_file()?;
            self.show_user_info_prompt = false;
            self.git_status = "User information saved successfully!".to_string();
        } else {
            return Err("All fields must be filled".into());
        }
        
        Ok(())
    }

    pub fn get_user_info_errors(&self) -> Vec<String> {
        let mut errors = Vec::new();
        
        if self.user_info.name.trim().is_empty() {
            errors.push("Name is required".to_string());
        }
        if self.user_info.employee_number.trim().is_empty() {
            errors.push("Employee Number is required".to_string());
        }
        if self.user_info.designation.trim().is_empty() {
            errors.push("Designation is required".to_string());
        }
        
        errors
    }
}
