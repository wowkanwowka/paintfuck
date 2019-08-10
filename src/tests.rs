#[cfg(test)]
use crate::interpreter::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_test() {
        let code = "*[eee]s*";
        let iterations = 10;
        let width = 3;
        let height = 3;
        let mut interpreter = Interpreter::new( code, iterations, width, height );
        for _i in 0..4 {
            interpreter.process_next().expect("simple test failed");
        }
        assert_eq!(interpreter.get_state(), "1 0 0 \n1 0 0 \n0 0 0\n");
    }

    #[test]
    fn empty_test() {

    }

    #[test]
    fn bigger_test() {

    }

    #[test]
    fn find_match_test() {
        {
            let text = "e[blahblahblah]".to_string();
            assert_eq!(
                find_match_wrapper(
                    &text,
                    '[',
                    ']',
                    1,
                    1).expect("we got lamed"),
                (text.len() - 1) as i64)
        }
        {
            let text = "bbcbba".to_string();
            assert_eq!(
                find_match_wrapper(
                    &text,
                    'a',
                    'c',
                    (text.len() - 1) as i64,
                    -1).expect(format!("we got lamed, text was {}", text).as_str()),
                2)
        }
        {
            let text = "e[[[hh]]]".to_string();
            assert_eq!(
                find_match_wrapper(
                    &text,
                    '[',
                    ']',
                    1,
                    1).expect("we got lamed"),
                (text.len() - 1) as i64);
            assert_eq!(
                find_match_wrapper(
                    &text,
                    '[',
                    ']',
                    2,
                    1).expect("we got lamed"),
                (text.len() - 2) as i64);
            assert_eq!(
                find_match_wrapper(
                    &text,
                    ']',
                    '[',
                    (text.len() - 1) as i64,
                    -1).expect("we got lamed"),
                1);
            assert_eq!(
                find_match_wrapper(
                    &text,
                    ']',
                    '[',
                    (text.len() - 2) as i64,
                    -1).expect("we got lamed"),
                2);
        }
    }
//
//    #[test]
//    pub fn run_tests() {
//        find_match_test();
////        simple_test();
////        empty_test();
////        bigger_test();
//    }
}