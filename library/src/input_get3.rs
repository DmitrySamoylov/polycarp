pub fn get<T>(iter: &mut dyn Iterator<Item = &str>) -> (T, T, T)
where
    T: std::str::FromStr + std::fmt::Display,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    (
        super::input::parse_next(iter),
        super::input::parse_next(iter),
        super::input::parse_next(iter),
    )
}
