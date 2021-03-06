// Visibility: off
pub fn check<S: FnOnce(crate::input::Input) -> String>(
    input: &str,
    expected_output: &str,
    solve: S,
) {
    let input = &input[1..]; // Crop first \n
    let expected_output = &expected_output[1..]; // Crop first \n

    let actual_output = solve(crate::input::Input {
        iter: &mut input.split_whitespace(),
    });

    println!("================");
    println!("Input:\n{}", input);
    println!("================");
    println!("Expected output:\n{}", expected_output);
    println!("================");
    println!("Actual output:\n{}", actual_output);
    println!("================");

    if expected_output != actual_output {
        panic!("Test failed")
    }
}
// Visibility: on
