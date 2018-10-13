extern crate coral;

use coral::repl::Repl;

#[cfg(test)]
mod repl {
    use super::*;

    #[test]
    fn test_sum() {
        let source_code = String::from("1 + 1");
        assert_eq!("2", Repl::eval_code(source_code));
    }
}
