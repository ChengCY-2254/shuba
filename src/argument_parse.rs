pub struct ArgsParse;

impl ArgsParse {
    pub fn parse(mut args: std::env::Args) -> Result<CliArguments, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let mut cli_args = CliArguments {
            txt_link: String::new(),
        };

        while let Some(arg) = args.next() {
            if arg == "-l" {
                cli_args.txt_link = args.next().expect("No argument for -l");
            }
        }
        Ok(cli_args)
    }
}
#[derive(Debug, Clone)]
pub struct CliArguments {
    pub txt_link: String,
}