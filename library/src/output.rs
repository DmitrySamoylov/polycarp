pub trait Output {
    fn print(&self, buf: &mut String);
    fn print_with_len(&self, buf: &mut String);
}

impl<T: std::fmt::Display> Output for Vec<T> {
    fn print(&self, buf: &mut String) {
        if let Some((last, elements)) = self.split_last() {
            for e in elements {
                *buf += &format!("{e} ");
            }
            *buf += &format!("{last}\n");
        }
    }

    fn print_with_len(&self, buf: &mut String) {
        *buf += &format!("{}\n", self.len());
        self.print(buf);
    }
}

// Visibility: off
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prints_one() {
        let v = vec![1];
        let mut ans = String::new();
        v.print(&mut ans);
        assert_eq!(ans, "1\n");
    }

    #[test]
    fn prints_multiple() {
        let v = vec![1, 2, 3];
        let mut ans = String::new();
        v.print(&mut ans);
        assert_eq!(ans, "1 2 3\n");
    }

    #[test]
    fn prints_multiple_with_len() {
        let v = vec![1, 2, 3];
        let mut ans = String::new();
        v.print_with_len(&mut ans);
        assert_eq!(ans, "3\n1 2 3\n");
    }
}
// Visibility: on
