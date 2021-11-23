pub fn get<T>(iter: &mut dyn Iterator<Item = &str>) -> T
where
    T: std::str::FromStr + std::fmt::Display,
{
    parse_next(iter)
}

pub fn parse_next<T>(iter: &mut dyn Iterator<Item = &str>) -> T
where
    T: std::str::FromStr + std::fmt::Display,
{
    parse_or_panic(iter.next().expect("No more items"))
}

pub fn parse_or_panic<T: std::str::FromStr>(s: &str) -> T {
    s.parse().unwrap_or_else(|_| {
        panic!(
            "Wrong type for {}, expected {}",
            s,
            std::any::type_name::<T>()
        )
    })
}
