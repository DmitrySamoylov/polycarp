use std::{
    any::type_name,
    fmt::{Debug, Display},
    str::FromStr,
};

pub struct Input<'a> {
    pub iter: &'a mut dyn Iterator<Item = &'a str>,
}

impl Input<'_> {
    pub fn get<T>(&mut self) -> T
    where
        T: FromStr + Display,
    {
        self.parse_next()
    }

    pub fn get2<T>(&mut self) -> (T, T)
    where
        T: FromStr + Display,
        <T as FromStr>::Err: Debug,
    {
        (self.parse_next(), self.parse_next())
    }

    pub fn get3<T>(&mut self) -> (T, T, T)
    where
        T: FromStr + Display,
        <T as FromStr>::Err: Debug,
    {
        (self.parse_next(), self.parse_next(), self.parse_next())
    }

    pub fn getn<T>(&mut self, n: usize) -> Vec<T>
    where
        T: FromStr + Display,
        <T as FromStr>::Err: Debug,
    {
        self.iter.take(n).map(just_parse).collect()
    }

    pub fn getn2<T>(&mut self, n: usize) -> Vec<(T, T)>
    where
        T: FromStr + Display,
        <T as FromStr>::Err: Debug,
    {
        self.iter
            .take(2 * n)
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|v| (just_parse(v[0]), just_parse(v[1])))
            .collect()
    }

    pub fn getn3<T>(&mut self, n: usize) -> Vec<(T, T, T)>
    where
        T: FromStr + Display,
        <T as FromStr>::Err: Debug,
    {
        self.iter
            .take(3 * n)
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|v| (just_parse(v[0]), just_parse(v[1]), just_parse(v[2])))
            .collect()
    }

    pub fn gets(&mut self) -> Vec<char> {
        self.get::<String>().chars().collect()
    }

    fn parse_next<T>(&mut self) -> T
    where
        T: FromStr + Display,
    {
        just_parse(self.iter.next().expect("No more items"))
    }
}

fn just_parse<T: FromStr>(s: &str) -> T {
    s.parse()
        .unwrap_or_else(|_| panic!("Wrong type for {}, expected {}", s, type_name::<T>()))
}
