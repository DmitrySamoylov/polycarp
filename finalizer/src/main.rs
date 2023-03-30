use std::{
    fs::File,
    io::{BufRead, Write},
};

use anyhow::Context;
use constants::{LIBRARY_DIR, SUBMISSION_DIR, SUBMISSION_INPUT_MAIN, SUBMISSION_OUTPUT};
use fn_error_context::context;
use utils::fs::{open_to_read, open_to_write};

fn main() {
    if let Err(e) = finalize_submission() {
        eprintln!("Failed to finalize submission: {:?}", e)
    }
}

fn finalize_submission() -> anyhow::Result<()> {
    let main = open_to_read(SUBMISSION_INPUT_MAIN)?;
    let mut submission = open_to_write(SUBMISSION_OUTPUT)?;
    let mut visible = true;

    for line in main.lines() {
        let line = line?;

        if line.starts_with("// Visibility: off") {
            visible = false;
            continue;
        }

        if line.starts_with("// Visibility: on") {
            visible = true;
            continue;
        }

        if let Some(module) = line.strip_prefix("mod ") {
            let module = module.strip_suffix(';').unwrap();

            append_module(
                &format!("{SUBMISSION_DIR}/{module}.rs"),
                module,
                &mut submission,
                "",
            )?;

            continue;
        }

        if visible {
            submission.write_fmt(format_args!("{line}\n"))?;
        }
    }

    println!("Submission is written to {SUBMISSION_OUTPUT}");

    Ok(())
}

#[context("Appending module: path={path}, module={module}")]
fn append_module(path: &str, module: &str, out_file: &mut File, ident: &str) -> anyhow::Result<()> {
    let reader = open_to_read(path)?;

    let mut visible = true;

    out_file.write_fmt(format_args!("{ident}mod {module} {{\n"))?;

    for line in reader.lines() {
        let line = line?;

        if line.starts_with("// Visibility: off") {
            visible = false;
            continue;
        }

        if line.starts_with("// Visibility: on") {
            visible = true;
            continue;
        }

        match line.strip_prefix("use library::") {
            Some(line) => {
                let lib_mods = match line.starts_with('{') {
                    false => vec![line.strip_suffix(';').unwrap()],
                    true => line
                        .strip_prefix('{')
                        .context("Bad import format '{'")?
                        .strip_suffix("};")
                        .context("Bad import format '}'")?
                        .split(", ")
                        .collect(),
                };

                for lib_mod in lib_mods {
                    append_module(
                        &format!("{LIBRARY_DIR}/{lib_mod}.rs"),
                        lib_mod,
                        out_file,
                        &format!("{ident}    "),
                    )?;
                }
            }
            None => {
                if visible {
                    out_file.write_fmt(format_args!("{ident}    {line}\n"))?;
                }
            }
        }
    }

    out_file.write_fmt(format_args!("{ident}}}\n"))?;

    Ok(())
}
