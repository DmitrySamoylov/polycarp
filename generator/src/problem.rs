use std::{
    fs::{copy, create_dir_all, read_to_string, File},
    io::{BufRead, BufReader, Write},
};

use anyhow::Context;
use fn_error_context::context;
use inflector::Inflector;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use utils::fs::{open_to_read, open_to_write};

const TEMPLATE_RS: &str = "submission/src/template.rs";
const SOLVE_RS: &str = "submission/src/solve.rs";
const CURR_JSON: &str = "data/curr.json";
const HISTORY_DIR: &str = "data/history";

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

    let template = open_to_read(TEMPLATE_RS)?;

    let mut out_file = open_to_write(SOLVE_RS)?;

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
                out_file.write_all(make_test_args(&t.input, "input").as_bytes())?;
                out_file.write_all(make_test_args(&t.output, "output").as_bytes())?;
                out_file.write_fmt(format_args!(
                    "        crate::check::check(input, output, super::solve);\n"
                ))?;
                out_file.write_fmt(format_args!("    }}\n"))?;
            }

            out_file.write_fmt(format_args!("}}\n"))?;
        } else {
            out_file.write_fmt(format_args!("{}\n", line))?;
        }
    }

    Ok(())
}

fn make_test_args(t: &str, var_name: &str) -> String {
    format!("        let {} = r#\"\n{}\"#;\n\n", var_name, t)
}

#[context("Storing problem to {}", CURR_JSON)]
fn store_problem(problem: &Problem) -> anyhow::Result<()> {
    serde_json::to_writer(&open_to_write(CURR_JSON)?, &problem)
        .with_context(|| format!("Serializing {:?}", problem))
}

#[context("Performing backup of previous problem solution: {}", SOLVE_RS)]
fn backup_solution_of_prev_problem() -> anyhow::Result<()> {
    create_dir_all(HISTORY_DIR).with_context(|| format!("Creating {}", HISTORY_DIR))?;

    if let Ok(curr_json) = read_to_string(CURR_JSON) {
        let prev_problem = from_str::<Problem>(&curr_json)
            .with_context(|| format!("Parsing previous problem from JSON: {}", curr_json))?;

        let dst = format!("{}/{}.rs", HISTORY_DIR, prev_problem.name.to_snake_case());

        copy(SOLVE_RS, &dst).with_context(|| format!("Copying {} to {}", SOLVE_RS, dst))?;
    }

    Ok(())
}
