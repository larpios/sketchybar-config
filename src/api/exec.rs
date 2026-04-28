use anyhow::Result;
use std::process::Command;

pub trait Executor {
    fn execute(&self, args: Vec<String>) -> Result<()>;
}

pub struct SketchybarExecutor;

impl Executor for SketchybarExecutor {
    fn execute(&self, args: Vec<String>) -> Result<()> {
        let output = Command::new("sketchybar").args(&args).output()?;
        if !output.status.success() {
            eprintln!("Error executing sketchybar command with args: {:?}", args);
            eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
        }
        Ok(())
    }
}

// Global executor for convenience, or we could pass it around.
// For now, let's keep it simple and have functions in src/api/mod.rs use a default one.
