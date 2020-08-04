use std::env;

pub fn env(var: String) -> String {
    // $SHELL and $USER are set automatically, the only env variable it would fail on is $EDITOR
    env::var(var).expect("Could not read $EDITOR, are you sure it's set?")
}
