pub fn get<T>(iter: &mut dyn Iterator<Item = &str>, n: usize) -> Vec<(T, T)>
where
    T: std::str::FromStr + std::fmt::Display,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    iter.take(2 * n)
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|v| {
            (
                super::input::parse_or_panic(v[0]),
                super::input::parse_or_panic(v[1]),
            )
        })
        .collect()
}
