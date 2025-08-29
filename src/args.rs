#[derive(Debug)]
pub struct AppArgs {
    pub input: String,
    pub output: String,
    pub options: (),
}

pub fn parse(mut args: impl Iterator<Item = String>) -> anyhow::Result<AppArgs> {
    const ERR_FORMAT: &str = "Missing or invalid {arg_name} arg. Format should be:\n`{app_name} -i <path to input directory> -o <path to output directory>`";
    const APP_NAME: &str = env!("CARGO_CRATE_NAME");

    let input_err = Err(anyhow::Error::msg(
        ERR_FORMAT
            .replace("{arg_name}", "input")
            .replace("{app_name}", APP_NAME),
    ));

    let output_err = Err(anyhow::Error::msg(
        ERR_FORMAT
            .replace("{arg_name}", "output")
            .replace("{app_name}", APP_NAME),
    ));

    let mut input = None;
    let mut output = None;

    'parse: loop {
        let Some(arg) = args.next() else {
            break 'parse;
        };

        if arg == "-i" {
            match args.next() {
                Some(value) => {
                    if value.starts_with("-") || input.is_some() {
                        return input_err;
                    }
                    input = Some(value.clone());
                }
                None => break 'parse,
            }
        }

        if arg == "-o" {
            match args.next() {
                Some(value) => {
                    if value.starts_with("-") || output.is_some() {
                        return output_err;
                    }
                    output = Some(value.clone());
                }
                None => break 'parse,
            }
        }
    }

    if input.is_none() {
        return input_err;
    }

    if output.is_none() {
        return output_err;
    }

    Ok(AppArgs {
        input: input.unwrap(),
        output: output.unwrap(),
        options: (),
    })
}
