pub struct Flags {
    pub allow_mangle: bool,
    pub verbose: bool,
    pub text: String,
}

pub fn parse_args(args: Vec<String>) -> Result<Flags, String> {
    let mut f = Flags {
        allow_mangle: false,
        verbose: false,
        text: String::from(""),
    };

    for (i, a) in args.iter().enumerate() {
        match a.clone().as_str() {
            "--allow-mangle" => f.allow_mangle = true,
            "--verbose" | "-v" => f.verbose = true,
            _ => {
                if i == args.clone().len() - 1 {
                    f.text = a.to_string();
                } else {
                    return Err(format!("'{}', is an unexpected flag", a.to_string()));
                }
            }
        }
    }

    Ok(f)
}
