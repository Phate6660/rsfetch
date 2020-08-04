use std::env;

pub fn env(var: String) -> String {
    env::var(var).unwrap()
}
