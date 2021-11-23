pub fn get<T>(iter: &mut dyn Iterator<Item = &str>, n: usize) -> Vec<T>
where
    T: std::str::FromStr + std::fmt::Display,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    iter.take(n).map(super::input::just_parse).collect()
}
