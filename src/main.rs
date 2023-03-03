use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Result;
use clap::Parser;
use git2::Repository;
use mlua::{Function, Lua, Table};

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub theme: String,
}

fn git_table(lua: &Lua) -> Result<Table> {
    let git = lua.create_table()?;
    git.set("ok", false)?;

    let repo = match Repository::open_from_env() {
        Ok(repo) => repo,
        Err(_) => {
            return Ok(git);
        }
    };

    git.set("ok", true)?;
    git.set(
        "workdir",
        match repo.workdir() {
            Some(path) => path.to_str().unwrap_or(""),
            None => "",
        },
    )?;

    // state
    git.set("state", format!("{:?}", repo.state()))?;

    // conflicts
    let index = repo.index()?;
    let conflicts = index.conflicts()?;
    git.set("conflicts", conflicts.count())?;

    Ok(git)
}

fn main() -> Result<()> {
    let args = Args::parse();
    let lua = Lua::new();

    let globals = lua.globals();

    let theme = PathBuf::from_str(&args.theme)?;
    let chunk = lua.load(&theme).set_name(args.theme)?;

    chunk.exec()?;

    let modules: Function = globals.get("Modules")?;
    let modules = modules.call::<_, Vec<String>>(())?;
    // println!("modules: {:?}", modules);

    let ctx = lua.create_table()?;
    if modules.contains(&String::from("git")) {
        ctx.set("git", git_table(&lua)?)?;
    }

    let prompt: Function = globals.get("Prompt")?;
    let env = lua.create_table()?;
    env.set("AWS_PROFILE", "aws-profile")?;


    ctx.set("env", env)?;
    ctx.set("username", "andy")?;
    let prompt = prompt.call::<_, String>(ctx)?;

    println!("{:?}", prompt);

    Ok(())
}
