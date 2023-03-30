use std::{
    fs::{copy, create_dir_all, read_to_string, File},
    io::{BufRead, BufReader, Write},
};

use anyhow::Context;
use constants::{CURR_JSON, HISTORY_DIR, SUBMISSION_INPUT_SOLVE, SUBMISSION_TEMPLATE};
use fn_error_context::context;
use heck::ToSnakeCase;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use utils::fs::{open_to_read, open_to_write};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    input: String,
    output: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Problem {
    name: String,
    group: String,
    tests: Vec<Test>,
}

#[context("Adding the problem")]
pub(crate) async fn add_problem(problem: Problem) -> anyhow::Result<()> {
    println!("Parsing problem {}", problem.name);

    backup_solution_of_prev_problem()?;

    let template = open_to_read(SUBMISSION_TEMPLATE)?;

    let mut out_file = open_to_write(SUBMISSION_INPUT_SOLVE)?;

    expand_template(&problem, template, &mut out_file)?;

    store_problem(&problem)?;

    println!("Problem parsed successfully!");

    Ok(())
}

#[context("Expanding the template")]
fn expand_template(
    problem: &Problem,
    template: BufReader<File>,
    out_file: &mut File,
) -> anyhow::Result<()> {
    for line in template.lines() {
        let line = line.context("Failed to get line")?;

        if line.starts_with("// Tests go here") {
            out_file.write_fmt(format_args!("#[cfg(test)]\n"))?;
            out_file.write_fmt(format_args!("mod tests {{\n"))?;

            for (i, t) in problem.tests.iter().enumerate() {
                out_file.write_fmt(format_args!("\n"))?;
                out_file.write_fmt(format_args!("    #[test]\n"))?;
                out_file.write_fmt(format_args!("    fn test_{}() {{\n", i + 1))?;
                out_file.write_all(make_test_args("input", &t.input).as_bytes())?;
                out_file.write_all(make_test_args("output", &t.output).as_bytes())?;
                out_file.write_fmt(format_args!(
                    "        crate::check::check(input, output, super::solve);\n"
                ))?;
                out_file.write_fmt(format_args!("    }}\n"))?;
            }

            out_file.write_fmt(format_args!("}}\n"))?;
        } else {
            out_file.write_fmt(format_args!("{line}\n"))?;
        }
    }

    Ok(())
}

fn make_test_args(var_name: &str, test_data: &str) -> String {
    format!("        let {var_name} = r#\"\n{test_data}\"#;\n\n")
}

#[context("Storing problem to {CURR_JSON}")]
fn store_problem(problem: &Problem) -> anyhow::Result<()> {
    serde_json::to_writer(&open_to_write(CURR_JSON)?, &problem)
        .with_context(|| format!("Serializing {:?}", problem))
}

#[context("Saving backup of previous problem solution")]
fn backup_solution_of_prev_problem() -> anyhow::Result<()> {
    create_dir_all(HISTORY_DIR).with_context(|| format!("Creating {HISTORY_DIR}"))?;

    if let Ok(curr_json) = read_to_string(CURR_JSON) {
        let prev_problem = from_str::<Problem>(&curr_json)
            .with_context(|| format!("Parsing previous problem from JSON: {curr_json}"))?;

        let dst = format!("{HISTORY_DIR}/{}.rs", prev_problem.name.to_snake_case());

        copy(SUBMISSION_INPUT_SOLVE, &dst)
            .with_context(|| format!("Copying {SUBMISSION_INPUT_SOLVE} to {dst}"))?;
    }

    Ok(())
}
