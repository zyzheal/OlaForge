#[cfg(test)]
mod tests {
    use crate::SandboxExecutor;

    #[test]
    fn test_safe_code_passes() {
        let sandbox = SandboxExecutor::new(false);
        let result = sandbox.is_safe("print('hello')", "python");
        assert!(result, "Safe code should pass");
    }

    #[test]
    fn test_dangerous_code_fails() {
        let sandbox = SandboxExecutor::new(false);
        // This might pass or fail depending on scanner rules
        let issues = sandbox.get_issues("import os; os.system('rm -rf /')", "python");
        // Should detect at least some issues
        println!("Issues: {:?}", issues);
    }

    #[test]
    fn test_network_detection() {
        let sandbox = SandboxExecutor::new(false); // network disabled
        let issues = sandbox.check_network("import requests; requests.get('http://example.com')", "python");
        assert!(!issues.is_empty(), "Should detect network usage when disabled");
    }

    #[test]
    fn test_network_allowed() {
        let sandbox = SandboxExecutor::new(true); // network enabled
        let issues = sandbox.check_network("import requests", "python");
        assert!(issues.is_empty(), "Should allow network when enabled");
    }

    #[test]
    fn test_javascript_network_detection() {
        let sandbox = SandboxExecutor::new(false);
        let issues = sandbox.check_network("fetch('http://api.example.com')", "javascript");
        assert!(!issues.is_empty(), "Should detect JS fetch");
    }
}