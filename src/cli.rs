pub mod cli {
    use clap::{ArgEnum, Parser};
    use std::fmt;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
    pub enum Mode {
        Beginner,
        Advanced,
    }

    impl fmt::Display for Mode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Mode::Beginner => write!(f, "beginner"),
                Mode::Advanced => write!(f, "advanced"),
            }
        }
    }

    #[derive(Parser, Debug)]
    #[clap(author, version,long_about = None)]
    pub struct Args {
        /// Beginner mode is raise/fold, advanced mode involves ratios
        #[clap(arg_enum, default_value_t = Mode::Beginner)]
        pub mode: Mode,
    }

}
