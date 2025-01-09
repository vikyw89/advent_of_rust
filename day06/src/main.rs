// Write a function that returns the reference to the longer string
// without any new allocations
pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    if s1.trim().chars().count() > s2.trim().chars().count() {
        Some(s1.trim())
    } else if s2.trim().chars().count() > s1.trim().chars().count() {
        Some(s2.trim())
    } else {
        None
    }
}

pub fn main() {}
