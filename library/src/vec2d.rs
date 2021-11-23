pub fn vec2d<T: Clone>(d1: usize, d2: usize, fill_value: T) -> Vec<Vec<T>> {
    vec![vec![fill_value; d2]; d1]
}

// Visibility: off
#[test]
fn test() {
    let v = vec2d(2, 3, 0);

    assert_eq!(v[0][0], 0);
    assert_eq!(v[0][1], 0);
    assert_eq!(v[0][2], 0);
    assert_eq!(v[1][0], 0);
    assert_eq!(v[1][1], 0);
    assert_eq!(v[1][2], 0);
}
// Visibility: on
