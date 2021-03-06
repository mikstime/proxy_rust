use async_std::prelude::*;
use crate::cli::Config;

fn parse_line(line: &str, config: &mut Config) -> (i32, i32, bool) {
    let mut start = config.history.start;
    let mut end = config.history.end;
    let split_line = line.split(" ").collect::<Vec<&str>>();
    let mut need_erase = false;
    if split_line.len() >= 3 {
        if split_line[1] == "page" {
            if split_line[2] == "+" {
                start += 10;
                end += 10;
            } else if split_line[2] == "-"{
                if start < 10 {
                    start = 0;
                    end = 10;
                } else {
                    start -= 10;
                    end -= 10;
                }
            } else {
                let num = match split_line[2].parse::<i32>() {
                    Ok(v) => v,
                    Err(_e) => {
                        println!("Параметр page может принимать значения +/-/int");
                        println!("Значение {} не является подходящим. Будет использовано предыдущее значение ({})", split_line[2], (start + 10) / 10);
                        (start + 10) / 10
                    }
                };
                start = num * 10 - 10;
                end = start + 10;
            }
        }
    } else if split_line.len() >= 2 && split_line[1] == "erase" {
        need_erase = true;
    }
    config.history.start = start.clone();
    config.history.end = end.clone();
    (start, end, need_erase)
}
pub async fn erase_history() -> std::io::Result<()> {
    use async_std::fs;

    fs::remove_dir_all("./requests").await?;
    fs::create_dir("./requests").await?;
    println!("История запросов успешно очищена");
    Ok(())
}
pub async fn show_requests(line: &str, config: &mut Config) -> std::io::Result<()> {

    let (start, end, need_erase) = parse_line(line, config);
    if need_erase {
        return erase_history().await;
    }
    let mut entries = async_std::fs::read_dir("./requests").await?;
    let mut cur = 0;

    let mut file_names: Vec<Vec<String>> = Vec::new();
    while let Some(res) = entries.next().await {
        let entry = res?;
        if cur >= start {
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();
            let split = file_name.split("|||").clone().collect::<Vec<&str>>();
            if split.len() < 4 { continue;}
            use chrono::{TimeZone, Utc};
            let dt = Utc.timestamp_millis( split[3].parse::<i64>().unwrap());
            let split1 = vec![split[0].to_string(), split[1].to_string(), split[2].to_string(), dt.to_rfc2822()];
            file_names.push(split1);
        }
        cur += 1;
        if cur > end { break;}
    }

    print_table(file_names, config).await;
    Ok(())
}
async fn print_table(list: Vec<Vec<String>>, config: &mut Config) {

    use cli_table::{Table, Row, Cell};
    use cli_table::format::{CellFormat, Justify};
    let bold = CellFormat::builder().bold(true).build();
    let mut rows = vec![
        Row::new(vec![
            Cell::new(&format!("Id"), bold),
            Cell::new(&format!("Method"), bold),
            Cell::new("Url", bold),
            Cell::new(&format!("page: {}", config.history.start / 10 + 1),
            CellFormat::builder().bold(true).justify(Justify::Right).build()
            ),
        ]),
    ];

    for item in list.iter() {
        rows.push(            Row::new(vec![
            Cell::new(&item[0], bold),
            Cell::new(&item[1], Default::default()),
            Cell::new(&item[2], Default::default()),
            Cell::new(&item[3], Default::default()),
        ]),)
    }
    let table = Table::new(rows, Default::default()).unwrap();
    if let Err(e) = table.print_stdout() {
        println!("Ошибка при отображении таблицы.");
        println!("Информация об ошибке: {}", e);
    }
}