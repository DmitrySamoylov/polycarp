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
                out_file.write_all(make_test_string_constants("input", &t.input, 2).as_bytes())?;
                out_file
                    .write_all(make_test_string_constants("output", &t.output, 2).as_bytes())?;
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

/// Assembles a Rust value containing a string with test data.
///
/// For example, if test data parsed from the task contains
///
/// ```
/// 2
/// 123
/// 456
/// ```
///
/// .. then this function would generate something like:
///
/// ```rust
/// let input = indoc::indoc! {r#"
///     2
///     123
///     456
/// "#};
/// ```
///
/// This value contains exactly the same string but wrapped in `indoc` to use in generated tests.
fn make_test_string_constants(var_name: &str, test_data: &str, indent: usize) -> String {
    let mut s = String::new();
    let tabs = " ".repeat(indent * 4);
    s += &format!("{tabs}let {var_name} = indoc::indoc! {{r#\"\n");

    for line in test_data.split("\n") {
        let tabs = " ".repeat((indent + 1) * 4);
        let line = line.trim_end();
        if !line.is_empty() {
            s += &format!("{tabs}{line}\n");
        }
    }

    s += &format!("{tabs}\"#}};\n\n");
    s
}

#[test]
fn make_test_args_is_correct() {
    let test_data = indoc::indoc! {"
        123
        456
        789
    "};

    let actual = make_test_string_constants("input", test_data, 2);
    let expected = r##"        let input = indoc::indoc! {r#"
            123
            456
            789
        "#};

"##;

    println!("test_data:\n'{test_data}'");
    println!("actual:\n'{actual}'");

    assert_eq!(actual, expected);
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
