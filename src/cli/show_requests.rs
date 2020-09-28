
pub async fn show_requests(_page: i32) {
    print_table().await;
}
async fn print_table() {
    use cli_table::{Table, Row, Cell};
    use cli_table::format::{CellFormat, Justify};
    let justify_right = CellFormat::builder().justify(Justify::Right).build();
    let bold = CellFormat::builder().bold(true).build();
    let table = Table::new(
        vec![
            Row::new(vec![
                Cell::new(&format!("Method"), bold),
                Cell::new("Url", bold),
            ]),
            Row::new(vec![
                Cell::new("GET", Default::default()),
                Cell::new("http://mail.ru/", justify_right),
            ]),
            Row::new(vec![
                Cell::new("GET", Default::default()),
                Cell::new("http://mail.ru/", justify_right),
            ]),
            Row::new(vec![
                Cell::new("GET", Default::default()),
                Cell::new("http://mail.ru/", justify_right),
            ]),
            Row::new(vec![
                Cell::new("GET", Default::default()),
                Cell::new("http://mail.ru/", justify_right),
            ]),
        ],
        Default::default(),
    ).unwrap();
    table.print_stdout();
}