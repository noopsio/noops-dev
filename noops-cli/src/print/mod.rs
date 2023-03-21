use colored::Colorize;
use prettytable::{format, Attr, Cell, Row, Table};
use std::io::{self, Write};

pub struct InteractiveTable(Table);

impl InteractiveTable {
    pub fn new(header: Vec<&str>, data: &Vec<Vec<String>>) -> Table {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_BOX_CHARS);

        let data_str: Vec<Vec<&str>> = data
            .iter()
            .map(|inner| inner.iter().map(|s| s.as_str()).collect::<Vec<&str>>())
            .collect();

        table.set_titles(Self::create_header(header));
        let rows = Self::create_entries(&data_str);

        for row in rows {
            table.add_row(row);
        }

        table
    }

    fn create_header(mut header: Vec<&str>) -> Row {
        header.insert(0, "Nr");
        Self::data_to_row(header, Some(prettytable::Attr::Bold))
    }

    fn create_entries(entries: &Vec<Vec<&str>>) -> Vec<Row> {
        let mut rows = entries
            .iter()
            .map(|row| Self::data_to_row(row.to_vec(), None))
            .collect::<Vec<Row>>();
            for (i, row) in rows.iter_mut().enumerate()  {
                row.insert_cell(i, Cell::new(&i.to_string()));
            }
        rows
    }

    fn data_to_row(data: Vec<&str>, style: Option<Attr>) -> Row {
        let cells;

        match style {
            Some(style) => {
                cells = data
                    .iter()
                    .map(|entry| Cell::new(entry).with_style(style))
                    .collect::<Vec<Cell>>()
            }
            None => {
                cells = data
                    .iter()
                    .map(|entry| Cell::new(entry))
                    .collect::<Vec<Cell>>()
            }
        }

        return Row::new(cells);
    }
}

pub enum Color {
    Red,
    Green,
    Blue,
    White,
}

impl Color {
    pub fn print_colorful(&self, text: &str) {
        match self {
            Color::Red => println!("{}", text.red()),
            Color::Green => println!("{}", text.green()),
            Color::Blue => println!("{}", text.blue()),
            Color::White => println!("{}", text.white()),
        }
    }
    pub fn prompt_text(color: &Color, question: &str) -> String {
        Color::print_colorful(color, question);
        io::stdout().flush().unwrap();

        // Read the user input
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read the user input");

        // Remove the newline character at the end of the input
        user_input = user_input.trim_end().to_string();
        user_input
    }

    pub fn prompt_number(color: &Color, question: &str) -> usize {
        let answer = Self::prompt_text(color, question);
        match answer.parse::<usize>() {
            Ok(value) => value,
            Err(_) => {
                println!("Please enter a valid number");
                return Self::prompt_number(color, question);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use prettytable::{Row, format, Table, Cell};
    use crate::print::InteractiveTable;
    
    #[test]
    fn test_table_generation() {
        let header = vec!["HEADER"];
        let data = vec![vec!["DATA".to_string()]];
        let table = InteractiveTable::new(header, &data);

        let mut wanted_table = Table::new();
        wanted_table.set_format(*format::consts::FORMAT_BOX_CHARS);
        let style = prettytable::Attr::Bold;
        let headers = Row::new(vec![Cell::new("Nr").with_style(style), Cell::new("HEADER").with_style(style)]);
        wanted_table.set_titles(headers);

        let data_row = Row::new(vec![Cell::new("0"), Cell::new("DATA")]);

        wanted_table.add_row(data_row);
        assert_eq!(table, wanted_table)
    }
}
