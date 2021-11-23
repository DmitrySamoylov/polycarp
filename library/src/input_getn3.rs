pub fn get<T>(iter: &mut dyn Iterator<Item = &str>, n: usize) -> Vec<(T, T, T)>
where
    T: std::str::FromStr + std::fmt::Display,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    iter.take(3 * n)
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|v| {
            (
                super::input::just_parse(v[0]),
                super::input::just_parse(v[1]),
                super::input::just_parse(v[2]),
            )
        })
        .collect()
}
