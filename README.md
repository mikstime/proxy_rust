### RUST Proxy

HTTP/HTTPS прокси с возможностью хранения HTTP запросов для повторной отправки и
 проверки на уязвимости.
 
В основе утилиты лежат библиотеки tokio, async_std и hyper.

Запуск утилиты 
```shell script
docker build -t rust_proxy . && docker run -i -p 1337:1337 rust_proxy
```
Посмотреть справку по консольной утилите
```shell script
help
```
Посмотреть историю запросов
```shell script
history
```
Навигация по истории
```shell script
history page +/-/int
```
Посмтреть сохраненный запрос
```shell script
request <id>
```
Отправить сохраненный запрос
```shell script
request <id> send
```
Проверить на уязвимости
```shell script
request <id> scan
```
Проверка происходит посредством подстановки строк
```
;cat /etc/passwd;
|cat /etc/passwd|
`cat /etc/passwd`
```
Во все заголовки, GET/POST параметры запроса.
