use std::env;

const APP_NAME: &str = "imabacon";

#[derive(Debug)]

pub(crate) struct AppArgs {
    pub(crate) input: String,
    pub(crate) output: String,
    pub(crate) options: (),
}

pub fn parse() -> anyhow::Result<AppArgs> {
    let mut input = None;
    let mut output = None;

    let mut iter = env::args().peekable();
    'parse: loop {
        let Some(arg) = iter.next() else {
            break 'parse;
        };

        if arg == "-i"
            && let Some(value) = iter.peek()
            && !value.starts_with("-")
        {
            input = Some(value.clone())
        }

        if arg == "-o"
            && let Some(value) = iter.peek()
            && !value.starts_with("-")
        {
            output = Some(value.clone())
        }
    }

    const ERR_FORMAT: &str = "Missing or invalid {arg_name} arg. Format should be:\n`{app_name} -i <path to input directory> -o <path to output directory>`";

    if input.is_none() {
        return Err(anyhow::Error::msg(
            ERR_FORMAT
                .replace("{arg_name}", "input")
                .replace("{app_name}", APP_NAME),
        ));
    }
    if output.is_none() {
        return Err(anyhow::Error::msg(
            ERR_FORMAT
                .replace("{arg_name}", "output")
                .replace("{app_name}", APP_NAME),
        ));
    }

    Ok(AppArgs {
        input: input.unwrap(),
        output: output.unwrap(),
        options: (),
    })
}
