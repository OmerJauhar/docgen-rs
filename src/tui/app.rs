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
}

impl App {
    pub fn new() -> Self {
        Self {
            current_step: FormStep::CodeFlow,
            input_buffer: String::new(),
        }
    }
}
