pub struct InputValidator;

impl InputValidator {
    pub fn validate_code(code: &str, max_length: usize) -> Result<(), String> {
        if code.is_empty() {
            return Err("代码不能为空".to_string());
        }
        
        if code.len() > max_length {
            return Err(format!("代码长度超过限制 (最大 {} 字符)", max_length));
        }
        
        // Check for null bytes
        if code.contains('\0') {
            return Err("代码包含无效字符".to_string());
        }
        
        Ok(())
    }
    
    pub fn validate_language(language: &str) -> Result<(), String> {
        let valid = ["python", "python3", "python2", "javascript", "js", "node",
                     "bash", "sh", "ruby", "go", "perl", "php"];
        
        if !valid.contains(&language.to_lowercase().as_str()) {
            return Err(format!("不支持的语言: {}，支持的: {:?}", language, valid));
        }
        
        Ok(())
    }
    
    pub fn validate_security_level(level: &str) -> Result<(), String> {
        let valid = ["L0", "L1", "L2", "L3", "disabled"];
        
        if !valid.contains(&level.to_uppercase().as_str()) && level != "disabled" {
            return Err(format!("无效安全级别: {}，有效值: {:?}", level, valid));
        }
        
        Ok(())
    }
    
    pub fn validate_timeout(timeout: u64, max_timeout: u64) -> Result<(), String> {
        if timeout == 0 {
            return Err("超时时间必须大于 0".to_string());
        }
        
        if timeout > max_timeout {
            return Err(format!("超时时间超过最大限制 (最大 {} 秒)", max_timeout));
        }
        
        Ok(())
    }
    
    pub fn validate_path(path: &str) -> Result<(), String> {
        // Check for path traversal attempts
        if path.contains("..") {
            return Err("无效路径: 不允许路径遍历".to_string());
        }
        
        // Check for null bytes
        if path.contains('\0') {
            return Err("路径包含无效字符".to_string());
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_code_valid() {
        assert!(InputValidator::validate_code("print('hello')", 10000).is_ok());
    }
    
    #[test]
    fn test_validate_code_empty() {
        assert!(InputValidator::validate_code("", 10000).is_err());
    }
    
    #[test]
    fn test_validate_code_too_long() {
        let long_code = "a".repeat(10001);
        assert!(InputValidator::validate_code(&long_code, 10000).is_err());
    }
    
    #[test]
    fn test_validate_language_valid() {
        assert!(InputValidator::validate_language("python").is_ok());
        assert!(InputValidator::validate_language("javascript").is_ok());
    }
    
    #[test]
    fn test_validate_language_invalid() {
        assert!(InputValidator::validate_language("invalid").is_err());
    }
    
    #[test]
    fn test_validate_security_level_valid() {
        assert!(InputValidator::validate_security_level("L0").is_ok());
        assert!(InputValidator::validate_security_level("L2").is_ok());
        assert!(InputValidator::validate_security_level("disabled").is_ok());
    }
    
    #[test]
    fn test_validate_security_level_invalid() {
        assert!(InputValidator::validate_security_level("L5").is_err());
    }
    
    #[test]
    fn test_validate_timeout_valid() {
        assert!(InputValidator::validate_timeout(60, 300).is_ok());
    }
    
    #[test]
    fn test_validate_timeout_zero() {
        assert!(InputValidator::validate_timeout(0, 300).is_err());
    }
    
    #[test]
    fn test_validate_timeout_too_long() {
        assert!(InputValidator::validate_timeout(500, 300).is_err());
    }
    
    #[test]
    fn test_validate_path_valid() {
        assert!(InputValidator::validate_path("/tmp/test").is_ok());
    }
    
    #[test]
    fn test_validate_path_traversal() {
        assert!(InputValidator::validate_path("../etc/passwd").is_err());
    }
}