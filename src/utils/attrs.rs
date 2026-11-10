use scraper::{Html, Selector};

pub fn consists_only_of_attributes(input: &str) -> bool {
    // 1. Оборачиваем строку в фиктивный тег
    let wrapped = format!("<div {}></div>", input);
    
    // 2. Парсим фрагмент
    let fragment = Html::parse_fragment(&wrapped);
    
    // 3. Ищем наш div
    let selector = Selector::parse("div").unwrap();
    let element = match fragment.select(&selector).next() {
        Some(el) => el,
        None => return false, // Если div не найден, значит парсинг сломался
    };
    
    // 4. Получаем все атрибуты
    let attrs: Vec<_> = element.value().attrs().collect();
    
    // 5. Проверяем, что атрибуты есть и они "непустые" (по имени)
    // Важно: html5ever прощает некоторые ошибки, но если в строке был мусор,
    // он может стать просто текстом внутри div, а не атрибутом.
    !attrs.is_empty() && attrs.iter().all(|(name, _)| !name.is_empty())
}