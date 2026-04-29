#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_interpreter_python() {
        assert_eq!(get_interpreter("python"), "python3");
        assert_eq!(get_interpreter("python3"), "python3");
        assert_eq!(get_interpreter("python2"), "python2");
    }

    #[test]
    fn test_get_interpreter_javascript() {
        assert_eq!(get_interpreter("javascript"), "node");
        assert_eq!(get_interpreter("js"), "node");
        assert_eq!(get_interpreter("node"), "node");
    }

    #[test]
    fn test_get_interpreter_shell() {
        assert_eq!(get_interpreter("bash"), "bash");
        assert_eq!(get_interpreter("sh"), "bash");
    }

    #[test]
    fn test_get_interpreter_other() {
        assert_eq!(get_interpreter("ruby"), "ruby");
        assert_eq!(get_interpreter("go"), "go");
        assert_eq!(get_interpreter("unknown"), "sh");
    }

    #[test]
    fn test_language_flag() {
        assert_eq!(get_language_flag("python"), "-c");
        assert_eq!(get_language_flag("javascript"), "-e");
        assert_eq!(get_language_flag("js"), "-e");
        assert_eq!(get_language_flag("bash"), "-c");
    }
}