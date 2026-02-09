use crate::data::generate_random_flights;
use chrono::NaiveDate;
use eframe::egui;
use eframe::App;
use egui::Sense;
use egui_extras::{Column, TableBuilder};
use itertools::Itertools;
use std::any::Any;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use crate::column_filters::{I32ColumnFilter, NaiveDateColumnFilter, StringColumnFilter, U32ColumnFilter, BoolColumnFilter};
use crate::table_filter::{ColumnFilter, TableFilter};

mod table_filter;
mod data;
mod column_filters;

#[derive(Clone)]
pub struct Flight {
    number: u32,
    orig: String,
    dest: String,
    dep_date: NaiveDate,
    mileage: u32,
    cancelled: RefCell<bool>,
    gate: RefCell<Option<String>>,
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };

    eframe::run_native(
        "Table Filter Demo",
        options,
        Box::new(|cc| {
            Ok(Box::<TableFilterApp>::default())
        }),
    )
}

struct TableFilterApp {
    flights: Rc<Vec<Flight>>,
    table_filter: Rc<TableFilter<Flight>>,
}

impl Default for TableFilterApp {
    fn default() -> Self {
        let flights = Rc::new(generate_random_flights(1_000));
        let table_filter = TableFilter::new(&flights);

        // STRING FILTERS
        string_filters!(
            table_filter,
            ("orig_filter", |x| x.orig.clone()),
            ("dest_filter", |x| x.dest.clone()),
            ("gate_number_filter", |x| x.gate.borrow().clone().unwrap_or_default()),
        );

        // NAIVE DATE FILTERS
        naive_date_filters!(
            table_filter,
            ("dep_date_filter", |x| x.dep_date),
        );

        // U32 FILTERS
        u32_filters!(
            table_filter,
            ("mileage_filter", |x| x.mileage),
        );

        // BOOL FILTERS
        bool_filters!(
            table_filter,
            ("cancelled_filter", |x| x.cancelled.borrow().clone())
        );

        Self {
            flights,
            table_filter
        }
    }
}

impl App for TableFilterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Flights");

            ui.style_mut().interaction.selectable_labels = false;

            // check for filter reset
            //self.column_filters.check_for_reset();

            TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .sense(Sense::click_and_drag())
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::remainder())
                .header(20.0, |mut header| {

                    // ORIG COLUMN
                    col_with_filter!(header, self.table_filter, "orig_filter", |ui| {
                        ui.strong("ORIG");
                        if self.table_filter.is_active_for_id("orig_filter") {
                            ui.strong("🌰");
                        }
                    });

                    // DEST COLUMN
                    col_with_filter!(header, self.table_filter, "dest_filter", |ui| {
                        ui.strong("DEST");
                        if self.table_filter.is_active_for_id("dest_filter") {
                            ui.strong("🌰");
                        }
                    });

                    // DEP DT COLUMN
                    col_with_filter!(header, self.table_filter, "dep_date_filter", |ui| {
                        ui.strong("DEP DATE");
                        if self.table_filter.is_active_for_id("dep_date_filter") {
                            ui.strong("🌰");
                        }
                    });

                    // MILEAGE COLUMN
                    col_with_filter!(header, self.table_filter, "mileage_filter", |ui| {
                        ui.strong("MILEAGE");
                        if self.table_filter.is_active_for_id("mileage_filter") {
                            ui.strong("🌰");
                        }
                    });

                    // CANCELLED COLUMN
                    col_with_filter!(header, self.table_filter, "cancelled_filter", |ui| {
                        ui.strong("CANCELLED");
                        if self.table_filter.is_active_for_id("cancelled_filter") {
                            ui.strong("🌰");
                        }
                    });

                    // GATE NUMBER COLUMN
                    col_with_filter!(header, self.table_filter, "gate_number_filter", |ui| {
                        ui.strong("GATE NUMBER");
                        if self.table_filter.is_active_for_id("gate_number_filter") {
                            ui.strong("🌰");
                        }
                    });

                })
                .body(|mut body| {
                    self.flights
                        .iter()
                        .filter(|flt| self.table_filter.evaluate(&flt))
                        .for_each(|mut flight| {
                        body.row(30.0, |mut row| {
                            row.col(|ui| {
                                ui.label(&flight.orig);
                            });
                            row.col(|ui| {
                                ui.label(&flight.dest);
                            });
                            row.col(|ui| {
                                ui.label(flight.dep_date.format("%-m/%-d/%Y").to_string());
                            });
                            row.col(|ui| {
                                ui.label(flight.mileage.to_string());
                            });
                            row.col(|ui| {
                                ui.checkbox(&mut flight.cancelled.borrow_mut(), "");
                            });
                            row.col(|ui| {
                                let mut option_proxy = flight.gate.borrow().clone().unwrap_or(String::default());
                                if ui.text_edit_singleline(&mut option_proxy).changed() {
                                    *flight.gate.borrow_mut() = if option_proxy.is_empty() { None } else { Some(option_proxy) };
                                }
                            });
                        });
                    });
                });
        });
    }
}