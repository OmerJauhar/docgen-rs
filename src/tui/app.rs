#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MainView {
    Form,
    GitDiff,
    LLMStats,
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
}

#[derive(Clone)]
pub enum FormStep {
    CodeFlow,
    DbChanges,
    Extensibility,
    PerfNotes,
}

pub struct App {
    pub current_step: FormStep,
    pub input_buffer: String,
    pub main_view: MainView,
    pub file_impacts: Vec<FileImpact>,
    pub token_stats: TokenStats,
    pub flow_step: usize, // 0..=4 for diagram
    pub current_branch: String,
    pub git_status: String,
    pub project_folder: String, // New field for selected folder
}

impl App {
    pub fn new() -> Self {
        Self {
            current_step: FormStep::CodeFlow,
            input_buffer: String::new(),
            main_view: MainView::Form,
            file_impacts: vec![
                FileImpact { filename: "src/main.rs".into(), lines_added: 12, lines_removed: 2, functions_modified: 1, impact_score: 8 },
                FileImpact { filename: "src/tui/layout.rs".into(), lines_added: 5, lines_removed: 0, functions_modified: 2, impact_score: 4 },
                FileImpact { filename: "src/tui/widgets.rs".into(), lines_added: 1, lines_removed: 1, functions_modified: 1, impact_score: 2 },
            ],
            token_stats: TokenStats { diff_tokens: 1200, form_tokens: 500, total_tokens: 1700 },
            flow_step: 0,
            current_branch: "main".to_string(),
            git_status: "3 files changed, 18 insertions(+), 4 deletions(-)".to_string(),
            project_folder: ".".to_string(), // Default to current dir
        }
    }
}
