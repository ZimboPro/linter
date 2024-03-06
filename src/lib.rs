pub mod hacker_news;
// pub mod hcl;
// pub mod hcl_json;
pub mod config;
pub mod hcl;
pub mod openapi;
pub mod util;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_of_values_are_equal() {
        let a: Vec<serde_json::Value> = vec![2.into(), true.into(), "Hello".into()];
        let b: Vec<serde_json::Value> = vec![true.into(), 2.into(), "Hello".into()];
        assert!(b.iter().all(|item| a.contains(item)));
        assert!(a.iter().all(|item| b.contains(item)));
    }
}
