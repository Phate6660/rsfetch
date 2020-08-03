use sedregex::find_and_replace;
use std::env;

pub fn env(var: String) -> String {
    let env_var = env::var(var).unwrap();
    find_and_replace(&env_var, &["s/\"//"]).unwrap().to_string()
}
