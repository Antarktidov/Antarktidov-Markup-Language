# Antarktidov Markup Language
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/Antarktidov/Antarktidov-Markup-Language/refs/heads/readme-and-license/readme-assets/logo-dark-theme.png?token=GHSAT0AAAAAAEMZBVXQWCXJFPW3IUE7QFWI2XURZVQ">
  <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/Antarktidov/Antarktidov-Markup-Language/refs/heads/readme-and-license/readme-assets/logo-light-theme.png?token=GHSAT0AAAAAAEMZBVXQNTB4ZZP6VQVHNX5O2XURZ6A">
  <img alt="Shows a black logo in light color mode and a white one in dark color mode." src="https://user-images.githubusercontent.com/25423296/163456779-a8556205-d0a5-45e2-ac17-42d089e3c3f8.png">
</picture>

Язык разметки, вдохновлённый вики-разметкой MediaWiki и языком Markdown. Компилируется в HTML.
## О языке
Данный язык я сделал, чтобы вести что-то вроде личных заметок. Взял то, что мне больше всего мне нравится в Markdown и Вики-разметке. И то, что смог, то и реализовал. Не судите строго, первый языке всё-таки. Но замечания и предложения по улучшению (если вы пользователь) и советы (если вы опытный разработчик языков) оставляйте в разделе [issues](https://github.com/Antarktidov/Antarktidov-Markup-Language/issues).
## Начало работы
Скачать язык можно со страницы [releases](https://github.com/Antarktidov/Antarktidov-Markup-Language/releases). Просто скачиваете exe-шник и всё. Чтобы можно было вызвать язык из любого места, нужно добавить путь к нему в переменную PATH. О том, как это сделать, см. на сайте [remontka.pro](https://remontka.pro/add-to-path-variable-windows/).
После добавления нужно в папке, где вы будете работать с языком, создать файл ``.env``.
Пример содержимого:
```env
domain=http://localhost
title=Заголовок
html_lang=ru
```
Теперь давайте создадим наш первый файл: ``hello.aml``
```markdown
# Hello
Привет, aml!
```
А теперь запустим команду:
```cmd
aml hello.aml hello.html
```
У нас получился html-файл*:
```html
<h1>Hello</h1>
<p>Привет, aml</p>
```

\* Здесь и далее показано только содержимое тэга body, если не указано иное, а сам html отформатирован, чтобы выглядеть человеко-читаемым.

Теперь рассмотрим синтаксис подробнее:
## Синтаксис
### Заголовки
Язык поддерживает заголовки как в Markdown:
```markdown
# Заголовок 1
## Заголовок 2
### Заголовок 3
#### Заголовок 4
##### Заголовок 5
###### Заголовок 6
```
как в вики-разметке:
```wikitext
= Заголовок 1 =
== Заголовок 2 ==
=== Заголовок 3 ===
==== Заголовок 4 ====
===== Заголовок 5 =====
====== Заголовок 6 ======
```
А также обычные html-заголовки:
> [!WARNING]  
> Тут баг

