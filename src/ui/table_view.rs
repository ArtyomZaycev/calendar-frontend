use std::{hash::Hash, marker::PhantomData};

use egui::{Button, Layout, Response};
use egui_extras::{Column, TableBuilder};
use serde::{Deserialize, Serialize};

pub trait TableViewItem {
    fn get_names() -> Vec<String>;
    fn get_fields(&self) -> Vec<String>;
}

#[derive(Debug, Clone, Copy)]
struct TableViewData {
    page: usize,
    page_size: usize,
}

impl Default for TableViewData {
    fn default() -> Self {
        Self {
            page: 0,
            page_size: 20,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TableView<T: TableViewItem> {
    id: egui::Id,
    phantom: PhantomData<T>,
}

impl<T: TableViewItem + Clone> Copy for TableView<T> {}

impl<T: TableViewItem> PartialEq for TableView<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
    fn ne(&self, other: &Self) -> bool {
        self.id != other.id
    }
}

impl<T: TableViewItem> TableView<T> {
    pub fn new(id: impl Hash) -> Self {
        Self {
            id: egui::Id::new(id),
            phantom: PhantomData,
        }
    }

    pub fn show(&self, ui: &mut egui::Ui, data: &Vec<T>) -> Response {
        let mut table_data = ui.memory(|memory| {
            memory
                .data
                .get_temp::<TableViewData>(self.id)
                .unwrap_or_default()
        });
        let response = ui
            .vertical(|ui| {
                self.show_table(ui, data, &mut table_data);
                self.show_page_switch(ui, data, &mut table_data);
            })
            .response;
        ui.memory_mut(|memory| {
            memory.data.insert_temp(self.id, table_data);
        });
        response
    }

    fn show_table(&self, ui: &mut egui::Ui, data: &Vec<T>, table_data: &mut TableViewData) {
        let columns = T::get_names();
        TableBuilder::new(ui)
            .columns(Column::auto().resizable(true), columns.len())
            .header(20.0, |mut header| {
                columns.into_iter().for_each(|name| {
                    header.col(|ui| {
                        ui.heading(name);
                    });
                });
            })
            .body(|mut body| {
                let first = table_data.page * table_data.page_size;
                (first..data.len().min(first + table_data.page_size)).for_each(|i| {
                    body.row(30.0, |mut row| {
                        data[i].get_fields().into_iter().for_each(|field| {
                            row.col(|ui| {
                                ui.label(field);
                            });
                        });
                    });
                });
            });
    }

    fn show_page_switch(
        &self,
        ui: &mut egui::Ui,
        data: &Vec<T>,
        table_data: &mut TableViewData,
    ) -> Response {
        ui.with_layout(Layout::right_to_left(egui::Align::TOP), |ui| {
            // RTL
            if ui
                .add_enabled(
                    (table_data.page + 1) * table_data.page_size < data.len(),
                    Button::new(">").small(),
                )
                .clicked()
            {
                table_data.page += 1;
            }
            /*let response = ui.text_edit_singleline(&mut self.page_str);
            if response.lost_focus() {
                match self.page_str.parse::<usize>() {
                    Ok(page) if page > 0 => self.page = page - 1,
                    _ => self.page_str = (self.page + 1).to_string(),
                }
            }*/
            if ui
                .add_enabled(table_data.page > 0, Button::new("<").small())
                .clicked()
            {
                table_data.page -= 1;
            }
        })
        .response
    }
}
