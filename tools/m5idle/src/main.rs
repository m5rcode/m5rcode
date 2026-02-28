/// m5idle - Terminal-based IDE for m5rcode v0.3.0
use std::io::{self, Write};
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let mut ide = IDE::new();
    ide.run();
}

struct IDE {
    current_file: Option<String>,
    content: String,
    modified: bool,
}

impl IDE {
    fn new() -> Self {
        IDE {
            current_file: None,
            content: String::new(),
            modified: false,
        }
    }
    
    fn run(&mut self) {
        self.print_banner();
        
        loop {
            self.print_menu();
            
            let choice = self.get_input("Choice: ");
            
            match choice.trim() {
                "1" => self.new_file(),
                "2" => self.open_file(),
                "3" => self.save_file(),
                "4" => self.save_as(),
                "5" => self.edit_file(),
                "6" => self.run_file(),
                "7" => self.run_repl(),
                "8" => self.show_help(),
                "9" | "q" | "quit" | "exit" => {
                    if self.confirm_exit() {
                        println!("👋 Goodbye!");
                        break;
                    }
                },
                _ => println!("❌ Invalid choice"),
            }
        }
    }
    
    fn print_banner(&self) {
        println!("\n╔═══════════════════════════════════════════════════════╗");
        println!("║            m5rcode IDE v0.3.0 (Terminal)             ║");
        println!("╚═══════════════════════════════════════════════════════╝\n");
    }
    
    fn print_menu(&self) {
        println!("\n┌─────────────────────────────────────────────────────┐");
        if let Some(ref file) = self.current_file {
            let status = if self.modified { " [modified]" } else { "" };
            println!("│ Current: {}{}", file, status);
        } else {
            println!("│ No file open");
        }
        println!("├─────────────────────────────────────────────────────┤");
        println!("│ 1. New File                                         │");
        println!("│ 2. Open File                                        │");
        println!("│ 3. Save File                                        │");
        println!("│ 4. Save As                                          │");
        println!("│ 5. Edit File                                        │");
        println!("│ 6. Run Current File                                 │");
        println!("│ 7. Launch REPL                                      │");
        println!("│ 8. Help                                             │");
        println!("│ 9. Exit                                             │");
        println!("└─────────────────────────────────────────────────────┘");
    }
    
    fn get_input(&self, prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input
    }
    
    fn new_file(&mut self) {
        if self.modified && !self.confirm_discard() {
            return;
        }
        
        self.current_file = None;
        self.content = String::new();
        self.modified = false;
        println!("✅ New file created");
    }
    
    fn open_file(&mut self) {
        if self.modified && !self.confirm_discard() {
            return;
        }
        
        let filename = self.get_input("Enter filename: ").trim().to_string();
        
        if filename.is_empty() {
            return;
        }
        
        match fs::read_to_string(&filename) {
            Ok(content) => {
                self.current_file = Some(filename.clone());
                self.content = content;
                self.modified = false;
                println!("✅ Opened: {}", filename);
                println!("\n📄 Content ({} lines):", self.content.lines().count());
                self.show_content_preview();
            },
            Err(e) => println!("❌ Error opening file: {}", e),
        }
    }
    
    fn save_file(&mut self) {
        if self.current_file.is_none() {
            self.save_as();
            return;
        }
        
        let filename = self.current_file.as_ref().unwrap();
        match fs::write(filename, &self.content) {
            Ok(_) => {
                self.modified = false;
                println!("✅ Saved: {}", filename);
            },
            Err(e) => println!("❌ Error saving file: {}", e),
        }
    }
    
    fn save_as(&mut self) {
        let filename = self.get_input("Enter filename: ").trim().to_string();
        
        if filename.is_empty() {
            return;
        }
        
        match fs::write(&filename, &self.content) {
            Ok(_) => {
                self.current_file = Some(filename.clone());
                self.modified = false;
                println!("✅ Saved as: {}", filename);
            },
            Err(e) => println!("❌ Error saving file: {}", e),
        }
    }
    
    fn edit_file(&mut self) {
        if self.current_file.is_none() {
            println!("❌ No file open. Create or open a file first.");
            return;
        }
        
        println!("\n📝 Simple Editor (Enter ':done' on a new line to finish)");
        println!("Current content:");
        self.show_content_preview();
        println!("\nEnter new content:");
        
        let mut new_content = String::new();
        loop {
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            
            if line.trim() == ":done" {
                break;
            }
            
            new_content.push_str(&line);
        }
        
        if !new_content.is_empty() {
            self.content = new_content;
            self.modified = true;
            println!("✅ Content updated");
        }
    }
    
    fn run_file(&self) {
        if self.current_file.is_none() {
            println!("❌ No file open");
            return;
        }
        
        if self.modified {
            println!("⚠️  File has unsaved changes");
        }
        
        let filename = self.current_file.as_ref().unwrap();
        println!("\n🚀 Running: {}\n", filename);
        println!("─────────────────────────────────────────────────────");
        
        let output = Command::new("m5repl")
            .arg(filename)
            .output();
        
        match output {
            Ok(result) => {
                print!("{}", String::from_utf8_lossy(&result.stdout));
                eprint!("{}", String::from_utf8_lossy(&result.stderr));
                println!("─────────────────────────────────────────────────────");
                if result.status.success() {
                    println!("✅ Execution completed");
                } else {
                    println!("❌ Execution failed");
                }
            },
            Err(e) => println!("❌ Error running file: {}", e),
        }
    }
    
    fn run_repl(&self) {
        println!("\n🚀 Launching REPL...\n");
        
        let status = Command::new("m5repl")
            .status();
        
        match status {
            Ok(_) => println!("\n✅ REPL closed"),
            Err(e) => println!("❌ Error launching REPL: {}", e),
        }
    }
    
    fn show_help(&self) {
        println!("\n📖 m5rcode IDE Help");
        println!("\nFeatures:");
        println!("  • Create, open, edit, and save .m5 files");
        println!("  • Run files directly");
        println!("  • Launch interactive REPL");
        println!("  • Simple terminal-based interface");
        println!("\nTips:");
        println!("  • Use :done to finish editing");
        println!("  • Files are saved with UTF-8 encoding");
        println!("  • Use m5repl for interactive testing");
        println!("\nFor full GUI IDE, use Kate or VS Code with m5rcode LSP");
    }
    
    fn show_content_preview(&self) {
        let lines: Vec<&str> = self.content.lines().collect();
        let preview_lines = lines.iter().take(10);
        
        for (i, line) in preview_lines.enumerate() {
            println!("{:3} | {}", i + 1, line);
        }
        
        if lines.len() > 10 {
            println!("... ({} more lines)", lines.len() - 10);
        }
    }
    
    fn confirm_discard(&self) -> bool {
        let response = self.get_input("⚠️  Discard unsaved changes? (y/n): ");
        response.trim().to_lowercase() == "y"
    }
    
    fn confirm_exit(&self) -> bool {
        if self.modified {
            let response = self.get_input("⚠️  Exit with unsaved changes? (y/n): ");
            response.trim().to_lowercase() == "y"
        } else {
            true
        }
    }
}
