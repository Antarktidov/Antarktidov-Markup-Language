use htmlparser::{Tokenizer, Token};
use std::collections::HashSet;

/// Возвращает true, если строка состоит ТОЛЬКО из разрешённых атрибутов
/// с валидными значениями.
pub fn is_only_allowed_attributes(input: &str) -> bool {
    let wrapped = format!("<div {}/>", input);

    let allowed: HashSet<&str> = ["class", "style", "border", "rowspan", "colspan"]
        .into_iter()
        .collect();

    let mut seen_any = false;

    for token in Tokenizer::from(wrapped.as_str()) {
        let token = match token {
            Ok(t) => t,
            Err(_) => return false, // ошибка парсинга — не наш случай
        };

        if let Token::Attribute { name, value, .. } = token {
            seen_any = true;
            let attr_name = name.as_str().to_lowercase();
            let attr_value = value.as_str();

            // 1. Имя должно быть в белом списке (отсекает danish, pie и т.п.)
            if !allowed.contains(attr_name.as_str()) {
                return false;
            }

            // 2. Валидация значения по типу атрибута
            let value_ok = match attr_name.as_str() {
                "class" => !attr_value.trim().is_empty(),
                "style" => is_valid_style(attr_value),
                "border" => {
                    attr_value.parse::<u32>().is_ok()
                        || matches!(attr_value, "thin" | "medium" | "thick")
                }
                "rowspan" | "colspan" => {
                    matches!(attr_value.parse::<u32>(), Ok(n) if n > 0)
                }
                _ => false,
            };

            if !value_ok {
                return false;
            }
        }
    }

    // Пустая строка — тоже не «только атрибуты»
    seen_any
}

/// Простая проверка CSS-деклараций в атрибуте style:
/// разбиваем по ';', каждый непустой кусок должен быть "prop: value".
fn is_valid_style(style: &str) -> bool {
    for decl in style.split(';') {
        let decl = decl.trim();
        if decl.is_empty() {
            continue;
        }
        let mut parts = decl.splitn(2, ':');
        let prop = parts.next().unwrap_or("").trim();
        let val = parts.next().unwrap_or("").trim();
        if prop.is_empty() || val.is_empty() {
            return false;
        }
    }
    true
}