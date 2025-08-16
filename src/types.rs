use std::fmt;
use clap::{ValueEnum, Parser};

#[derive(Parser, Debug, ValueEnum, Clone, Copy)]
pub enum Language {
    Py,
    Python,
    Rs,
    Rust,
    C,
    EmbedC,
    Cpp,
    Esp32,
    EmbdedRs,
    Arduino,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let variant_str = match self {
            Language::Python | Language::Py => "python",
            Language::Rs | Language::Rust => "rust",
            Language::C => "c",
            Language::Cpp => "cpp",
            Language::Esp32 | Language::EmbedC => "embedded-c", // esp-idf
            Language::EmbdedRs => "embedded-rust", 
            Language::Arduino => "embedded-cpp",
        };
        write!(f, "{}", variant_str)
    }
}

#[derive(Parser, Debug, ValueEnum, Clone, Copy)]
pub enum ProjectType{
    Personal,
    School,
    Work,
    Test,
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let variant_str = match self {
            ProjectType::Personal => "personal",
            ProjectType::School => "school",
            ProjectType::Work => "work",
            ProjectType::Test => "test",
        };
        write!(f, "{}", variant_str)
    }
}
