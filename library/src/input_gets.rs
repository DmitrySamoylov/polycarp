pub fn get(iter: &mut dyn Iterator<Item = &str>) -> Vec<char> {
    super::input::get::<String>(iter).chars().collect()
}
