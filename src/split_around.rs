
pub trait SplitAround {
    fn split_around(&self, index: usize) -> (String, String);
}

impl SplitAround for String {
    fn split_around(&self, index: usize) -> (String, String) {
        let before = self[..index].to_string();
        let after = self[index + 1..].to_string();
        (before, after)
    }
}