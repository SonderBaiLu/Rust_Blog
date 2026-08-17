use validator::ValidationError;

/// 1. 用户名非法字符校验（仅允许字母、数字、中文、下划线 `_` 和减号 `-`）
pub fn validate_username(username: &str) -> Result<(), ValidationError> {
    let is_valid = username
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-');

    if is_valid {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_username_characters"))
    }
}

/// 2. 手机号格式校验（中国大陆 11 位手机号：13x-19x）
pub fn validate_phone_number(phone: &str) -> Result<(), ValidationError> {
    let mut chars = phone.chars();
    let is_valid = phone.len() == 11
        && chars.next() == Some('1')
        && chars.next().map_or(false, |c| ('3'..='9').contains(&c))
        && chars.all(|c| c.is_ascii_digit());

    if is_valid {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_phone_number"))
    }
}

/// 3. 邮箱格式校验（若不完全依赖 validator 默认注解时可使用）
pub fn validate_email_format(email: &str) -> Result<(), ValidationError> {
    if let Some(at_idx) = email.find('@') {
        let domain = &email[at_idx + 1..];
        if at_idx > 0 && domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.')
        {
            return Ok(());
        }
    }
    Err(ValidationError::new("invalid_email_format"))
}
