fn sss(acc: Vec<char>, item: char) -> Vec<char> {
    Vec::new()
}

pub fn brackets_are_balanced(string: &str) -> bool {
    match string.chars().try_fold(Vec::new(), sss) {
        () => false,
        _ => true
    }
}
