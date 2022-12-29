use clap::Parser;

#[derive(Parser)]
pub struct Common {
    pub file: String,
}

#[derive(Parser)]
pub struct Grep {
    pub pattern: String,
    pub file: Option<String>,
    #[arg(short, long)]
    pub after: Option<usize>,
    #[arg(short, long)]
    pub before: Option<usize>
}

#[derive(Parser)]
pub struct Ls {
    pub dir: Option<String>,
    #[arg(short, long)]
    pub list: bool,
    #[arg(short, long)]
    pub all: bool,
}

#[derive(Parser)]
pub struct Tail {
    pub file: String,
    #[arg(short, long)]
    pub lines: Option<usize>,
}

impl Common {
    pub fn args() -> Self {
        let cli = Common::parse();
        Common { file: cli.file }
    }
}

impl Grep {
    pub fn args() -> Self {
        let cli = Grep::parse();
        let after = match cli.after {
            Some(after) => after,
            None => 0,
        };

        let before = match cli.before {
            Some(before) => before,
            None => 0,
        };

        Self {
            file: cli.file,
            pattern: cli.pattern,
            before: Some(before),
            after: Some(after),
        }
    }
}

impl Ls {
    pub fn args() -> Self {
        let cli = Ls::parse();
        let dir = if let Some(dir) = cli.dir { dir } else { String::from(".") };
        Self {
            dir: Some(dir),
            all: cli.all,
            list: cli.list,
        }
    }
}

impl Tail {
    pub fn args() -> Self {
        let cli = Tail::parse();
        Self {
            file: cli.file,
            lines: cli.lines,
        }
    }
}
