mod tui;
mod git_diff;
mod git_scan;
mod llm;

use clap::{Arg, Command};
use std::io;
use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    event::Event,
};
use ratatui::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("docgen")
        .version("1.0.0")
        .author("Omer Jauhar <omer.jauhar@greybeardsupport.com>")
        .about("AI-powered documentation generator for code changes - GreyBeard Outsourcing Internal Tool")
        .long_about("DocGen helps software engineers at GreyBeard Outsourcing generate comprehensive documentation for their code changes using AI analysis of git diffs and contextual information.")
        .subcommand(
            Command::new("generate")
                .about("Launch the interactive documentation generator")
                .arg(
                    Arg::new("project")
                        .short('p')
                        .long("project")
                        .value_name("PATH")
                        .help("Specify the project directory (defaults to current directory)")
                )
        )
        .subcommand(
            Command::new("config")
                .about("Configure user information")
        )
        .subcommand(
            Command::new("version")
                .about("Show version information")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("generate", sub_matches)) => {
            let project_path = sub_matches
                .get_one::<String>("project")
                .map(|s| s.as_str())
                .unwrap_or(".");
            
            launch_tui(project_path)?;
        }
        Some(("config", _)) => {
            launch_user_config()?;
        }
        Some(("version", _)) => {
            print_version_info();
        }
        _ => {
            // Default behavior - launch TUI
            launch_tui(".")?;
        }
    }

    Ok(())
}

fn launch_tui(project_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Terminal Setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = tui::app::App::new();
    
    // Set project folder if specified
    if project_path != "." {
        app.project_folder = project_path.to_string();
        if let Err(e) = app.load_branches(project_path) {
            app.git_status = format!("Error loading branches from {}: {}", project_path, e);
        }
    }

    let result = run_tui_loop(&mut terminal, &mut app);

    // Terminal cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_tui_loop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut tui::app::App) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|f| tui::layout::render_ui(f, app))?;
    
        if let Event::Key(key) = crossterm::event::read()? {
            if !tui::events::handle_input(key, app)? {
                break;
            }
        }
    }
    Ok(())
}

fn launch_user_config() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 DocGen User Configuration");
    println!("============================");
    
    let user_info = tui::app::UserInfo::load_from_file();
    
    if user_info.is_configured {
        println!("Current configuration:");
        println!("Name: {}", user_info.name);
        println!("Employee Number: {}", user_info.employee_number);
        println!("Designation: {}", user_info.designation);
        println!("\nUse 'docgen generate' and press Ctrl+E to modify settings interactively.");
    } else {
        println!("No user configuration found.");
        println!("Run 'docgen generate' and press Ctrl+E to set up your information.");
    }
    
    Ok(())
}

fn print_version_info() {
    println!("DocGen v1.0.0");
    println!("AI-powered documentation generator");
    println!("GreyBeard Outsourcing - Internal Tool");
    println!("Maintainer: Omer Jauhar <omer.jauhar@greybeardsupport.com>");
    println!("Built with Rust and ❤️");
}
