use std::collections::HashMap;

use clap::Parser;

#[derive(Parser)]
pub struct Common {
    file: String,
}

#[derive(Parser)]
pub struct Grep {
    pattern: String,
    file: Option<String>,
    #[arg(short, long)]
    after: Option<u32>,
    #[arg(short, long)]
    before: Option<u32>
}

#[derive(Parser)]
pub struct Ls {
    file: String,
    #[arg(short, long)]
    list: bool,
    #[arg(short, long)]
    all: bool,
}

impl Common {
    pub fn args() -> HashMap<String, String> {
        let cli = Common::parse();
        let mut args: HashMap<String, String> = HashMap::new();
        args.insert(String::from("file"), cli.file);
        args
    }
}

impl Grep {
    pub fn new() -> Self {
        Grep::parse()
    }

    pub fn args(self) -> HashMap<String, String> {
        let mut args: HashMap<String, String> = HashMap::new();
        if let Some(file) = self.file {
            args.insert(String::from("file"), file);
        }

        args.insert(String::from("pattern"), self.pattern);
        args
    }

    pub fn options(&self) -> HashMap<String, u32> {
        let after = match self.after {
            Some(after) => after,
            None => 0,
        };

        let before = match self.before {
            Some(before) => before,
            None => 0,
        };

        let mut options: HashMap<String, u32> = HashMap::new();
        options.insert(String::from("before"), before);
        options.insert(String::from("after"), after);
        options
    }
}

impl Ls {
    pub fn new() -> Self {
        Ls::parse()
    }
    
    pub fn args(self) -> HashMap<String, String> {
        let mut args: HashMap<String, String> = HashMap::new();
        args.insert(String::from("file"), self.file);
        args
    }

    pub fn options(&self) -> HashMap<String, bool> {
        let mut options: HashMap<String, bool> = HashMap::new();
        options.insert(String::from("list"), self.list);
        options.insert(String::from("all"), self.all);
        options
    }
}

