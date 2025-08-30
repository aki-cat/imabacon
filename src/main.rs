use std::env;

fn main() -> anyhow::Result<()> {
    imabacon::run(imabacon::args::parse(env::args())?)
}
