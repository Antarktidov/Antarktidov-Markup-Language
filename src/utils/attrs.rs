use htmlparser::{Tokenizer, Token};
use css_inspector::{validate_css_declarations_text, Config};
use std::collections::HashSet;

/// Проверяет, состоит ли строка только из разрешённых атрибутов с валидными значениями.
pub fn validate_specific_attributes(input: &str) -> Result<(), String> {
    // 1. Оборачиваем строку в фиктивный тег, чтобы htmlparser корректно её распознал
    let wrapped = format!("<div {}/>", input);
    let tokenizer = Tokenizer::from(wrapped.as_str());
    
    let allowed_attrs: HashSet<&str> = ["class", "style", "border", "rowspan", "colspan"].into_iter().collect();
    
    // 2. Итерируемся по токенам
    for token in tokenizer {
        match token {
            Ok(Token::Attribute { name, value, .. }) => {
                let attr_name = name.as_str().to_lowercase();
                
                // 3. Проверяем имя атрибута по белому списку
                if !allowed_attrs.contains(attr_name.as_str()) {
                    return Err(format!("Недопустимый атрибут: '{}'", attr_name));
                }
                
                let attr_value = value.as_str();
                
                // 4. Специфичная валидация значений
                match attr_name.as_str() {
                    "style" => {
                        // Используем css_inspector для проверки CSS-деклараций
                        let config = Config::default();
                        let report = validate_css_declarations_text(attr_value, &config);
                        if report.has_errors() {
                            return Err(format!("Невалидный CSS в атрибуте style: {:?}", report.messages));
                        }
                    }
                    "border" => {
                        // border может быть числом или одним из ключевых слов
                        let is_valid = attr_value.parse::<u32>().is_ok() 
                            || ["thin", "medium", "thick"].contains(&attr_value);
                        if !is_valid {
                            return Err(format!("Невалидное значение для border: '{}'", attr_value));
                        }
                    }
                    "rowspan" | "colspan" => {
                        // Должно быть положительным целым числом
                        if attr_value.parse::<u32>().is_err() || attr_value == "0" {
                            return Err(format!("Невалидное значение для {}: '{}'", attr_name, attr_value));
                        }
                    }
                    "class" => {
                        // Простая проверка: не пустая строка и нет запрещённых символов
                        if attr_value.trim().is_empty() {
                            return Err("Атрибут class не может быть пустым".to_string());
                        }
                        // При желании здесь можно добавить более строгую проверку на допустимые символы
                    }
                    _ => {}
                }
            }
            Ok(_) => {} // Игнорируем другие токены (открывающие/закрывающие теги)
            Err(e) => return Err(format!("Ошибка парсинга: {:?}", e)),
        }
    }
    
    Ok(())
}