pub fn get<T>(iter: &mut dyn Iterator<Item = &str>) -> (T, T)
where
    T: std::str::FromStr + std::fmt::Display,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let a = super::input::parse_next(iter);
    let b = super::input::parse_next(iter);
    (a, b)
}
