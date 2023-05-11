use cli_table::{Cell, CellStruct, Style, Table};

use crate::database::models::{Balance, Movement};
use crate::serializers::PersonOut;

pub fn print_person(people: Vec<PersonOut>, exclude: Vec<&str>) {
    let mut table_content: Vec<Vec<CellStruct>> = Vec::new();
    let mut table_head: Vec<CellStruct> = Vec::new();
    let mut fields = Vec::new();

    people.iter().for_each(|person| {
        let mut cell_struct_vec: Vec<CellStruct> = Vec::new();
        fields = person.fields();
        fields.iter().for_each(|field| {
            if !exclude.contains(&field.as_str()) {
                cell_struct_vec.push(person.get(&field).cell());
            }
        });

        table_content.push(cell_struct_vec)
    });

    fields.iter().for_each(|field| {
        if !exclude.contains(&field.as_str()) {
            table_head.push((&field).cell().bold(true));
        }
    });

    let table = table_content.table().title(table_head).bold(true);

    let table_display = table.display().unwrap();

    println!("{}", table_display);
}

pub fn print_statement(balance: Balance, movements: Vec<Movement>) {
    let mut table_content: Vec<Vec<CellStruct>> = Vec::new();
    let table_head: Vec<CellStruct> = vec!["date".cell(), "value".cell(), "actor".cell()];

    movements.iter().for_each(|movement| {
        let cell_struct_vec = vec![
            movement.date.to_string().cell(),
            movement.value.to_string().cell(),
            movement.actor.clone().cell(),
        ];
        table_content.push(cell_struct_vec);
    });

    table_content.push(vec![
        "TOTAL".cell(),
        balance.value.to_string().cell(),
        "".cell(),
    ]);

    let table = table_content.table().title(table_head).bold(true);
    let table_display = table.display().unwrap();

    println!("{}", table_display);
}
