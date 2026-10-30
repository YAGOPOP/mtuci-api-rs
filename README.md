# mtuci-api-rs

Неофициальный асинхронный Rust-клиент для API расписания [МТУСИ](https://mtuci.ru/).

Библиотека позволяет искать учебные группы, преподавателей и аудитории, а также получать расписание на выбранный месяц. Ответы API представлены типизированными структурами Rust.

## Возможности

- Поиск учебных групп, преподавателей и аудиторий.
- Получение расписания найденного объекта на выбранный месяц.
- Асинхронные HTTP-запросы на основе `reqwest`.
- Десериализация ответов API в типизированные структуры с помощью Serde.
- Возможность использовать собственный `reqwest::Client`.

## Установка

Добавьте зависимость в проект:

```bash
cargo add mtuci-api
```

Для примеров ниже также понадобится асинхронная среда выполнения Tokio:

```bash
cargo add tokio --features macros,rt-multi-thread
```

## Использование

### Поиск и получение расписания

```rust
use mtuci_api::{Client, ScheduleRequest, SearchRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // Ищем группу, преподавателя или аудиторию.
    let results = client
        .search(SearchRequest::new("БВТ2401"))
        .await?;

    for item in &results {
        println!("{} ({:?})", item.name(), item.kind());
    }

    // Если найдены результаты, получаем расписание первого.
    if let Some(item) = results.first() {
        let request = ScheduleRequest::for_item_current_month(item);
        let schedule = client.get_schedule(request).await?;

        for (date, classes) in schedule.days() {
            println!("\n{date}");

            for class in classes {
                println!(
                    "{}–{} — {} ({})",
                    class.time_start(),
                    class.time_end(),
                    class.discipline(),
                    class.class_type(),
                );
            }
        }
    }

    Ok(())
}
```

Метод `search()` возвращает список найденных объектов. Результат содержит название, тип объекта и значение, которое используется при запросе расписания.

`ScheduleRequest::for_item_current_month()` формирует запрос для найденного объекта на текущий месяц. Чтобы запросить другой месяц, используйте `ScheduleRequest::for_item()`.

### Использование собственного HTTP-клиента

Если требуется настроить тайм-ауты, прокси или другие параметры HTTP-соединения, можно передать готовый `reqwest::Client`:

```rust
use mtuci_api::Client;
use std::time::Duration;

fn create_client() -> Result<Client, reqwest::Error> {
    let http = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    Ok(Client::from_client(http))
}
```

Библиотека самостоятельно устанавливает необходимый `User-Agent` для запросов к API, поэтому отдельно настраивать этот заголовок в переданном HTTP-клиенте не требуется.

## Обработка ошибок

Методы, выполняющие запросы к API, возвращают `Result`. Ошибки представлены типом `mtuci_api::Error`, который позволяет различать ошибки HTTP-запросов, ошибки, возвращённые API, и отсутствие ожидаемых данных в ответе.

```rust
use mtuci_api::{Client, Error, SearchRequest};

async fn search_group() -> Result<(), Error> {
    let client = Client::new();
    let results = client
        .search(SearchRequest::new("БСТ2501"))
        .await?;

    println!("Найдено объектов: {}", results.len());

    Ok(())
}
```

## Особенности

- API не является официально документированным публичным интерфейсом библиотеки. Его поведение определяется текущей реализацией сайта МТУСИ.
- Формат ответов, доступность методов и правила обработки запросов могут изменяться без предварительного уведомления.
- Месяцы передаются в API с нумерацией от нуля, но публичные конструкторы запросов используют `chrono::Month`.
- Для получения расписания рекомендуется использовать `SearchValue`, полученный из результата поиска, а не создавать значение вручную.

## Ограничения и ответственность

Проект является независимой разработкой и не связан с администрацией МТУСИ. Он не гарантирует постоянную доступность API или актуальность данных, возвращаемых сервером.

Используйте библиотеку с учётом правил доступа к сайту и не создавайте чрезмерную нагрузку на его инфраструктуру.

## Документация

Документация публичного API доступна через Rustdoc:

```bash
cargo doc --open
```

Исходный код и сообщения об ошибках: [GitHub](https://github.com/YAGOPOP/mtuci-api-rs).
