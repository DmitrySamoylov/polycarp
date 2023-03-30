use const_format::formatcp;

pub const SUBMISSION_DIR: &str = "submission/src";
pub const SUBMISSION_TEMPLATE: &str = formatcp!("{SUBMISSION_DIR}/template.rs");
pub const SUBMISSION_INPUT_MAIN: &str = formatcp!("{SUBMISSION_DIR}/main.rs");
pub const SUBMISSION_INPUT_SOLVE: &str = formatcp!("{SUBMISSION_DIR}/solve.rs");
pub const SUBMISSION_OUTPUT: &str = formatcp!("{SUBMISSION_DIR}/submission.rs");

pub const LIBRARY_DIR: &str = "library/src";

pub const CURR_JSON: &str = "data/curr.json";
pub const HISTORY_DIR: &str = "data/history";
