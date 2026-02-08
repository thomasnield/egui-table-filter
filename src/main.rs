use std::any::Any;
use std::collections::HashSet;
use std::error::Error;
use std::rc::Rc;
use eframe::egui;
use eframe::App;
use chrono::NaiveDate;
use egui::{Id, Sense};
use egui_extras::{TableBuilder, Column};
use itertools::Itertools;
use crate::data::generate_random_flights;
use crate::table_filter::{ColumnFilter, ColumnFilterState, ScalarValue, TableFilter};

mod table_filter;
mod data;

#[derive(Clone)]
pub struct Flight {
    number: u32,
    orig: String,
    dest: String,
    dep_date: NaiveDate,
    mileage: u32,
    cancelled: bool,
    gate: Option<String>,
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

/*
impl Default for ColumnFilters {
    fn default() -> Self {
        Self {
            orig: ColumnFilterBridge::new(
                |flt: &Flight| flt.orig.clone(),
                |flt| flt.orig.clone(),
                |pattern, target| target.starts_with(pattern),
            ),
            dest: ColumnFilterBridge::new(
                |flt: &Flight| flt.dest.clone(),
                |flt| flt.dest.clone(),
                |pattern, target| target.starts_with(pattern),
            ),
            dep_date: ColumnFilterBridge::new(
                |flt: &Flight| flt.dep_date.format("%m/%d/%Y").to_string(),
                |flt| flt.dep_date.format("%m/%d/%Y").to_string(),
                |pattern, target|  {

                    return if Regex::new("[0-9]{1,2}/[0-9]{2}/[0-9]{4}><[0-9]{1,2}/[0-9]{2}/[0-9]{4}").unwrap().is_match(pattern) {
                        let (dt_left,dt_right) = pattern.split("><").collect_tuple().unwrap();

                        let dt_left = NaiveDate::parse_from_str(&dt_left, "%m/%d/%Y");
                        let dt_right = NaiveDate::parse_from_str(&dt_right, "%m/%d/%Y");
                        let dt = NaiveDate::parse_from_str(target, "%m/%d/%Y");

                        if let Ok(dt_left) = dt_left {
                            if let Ok(dt_right) = dt_right {
                                if let Ok(dt) = dt {
                                    dt_left <= dt && dt <= dt_right
                                } else {
                                    false
                                }
                            } else { false }
                        } else { false }
                    }
                    else if Regex::new("<[0-9]{1,2}/[0-9]{2}/[0-9]{4}").unwrap().is_match(pattern) {
                        let p = pattern.replace("<", "");

                        if let Ok(p) = NaiveDate::parse_from_str(&p, "%m/%d/%Y") {
                            if let Ok(s) = NaiveDate::parse_from_str(&target, "%m/%d/%Y") {
                                s <= p
                            } else { false }
                        } else { false }
                    } else if Regex::new(">[0-9]{1,2}/[0-9]{2}/[0-9]{4}").unwrap().is_match(pattern) {

                        let p = pattern.replace(">", "");

                        if let Ok(p) = NaiveDate::parse_from_str(&p, "%m/%d/%Y") {
                            if let Ok(s) = NaiveDate::parse_from_str(&target, "%m/%d/%Y") {
                                s >= p
                            } else { false }
                        } else { false }
                    }
                    else {
                        target.starts_with(pattern)
                    }
                },
            ),
            mileage: ColumnFilterBridge::new(
                |flt: &Flight| flt.mileage,
                |flt| flt.mileage.to_string(),
                |pattern, target|
                    return if Regex::new("[0-9]+><[0-9]+").unwrap().is_match(pattern) {
                        let (int_left, int_right) = pattern.split("><").collect_tuple().unwrap();

                        let u_left = int_left.parse::<usize>();
                        let u_right = int_right.parse::<usize>();
                        let u = target.parse::<usize>();

                        if let Ok(u_left) = u_left {
                            if let Ok(u_right) = u_right {
                                if let Ok(u) = u {
                                    u_left <= u && u <= u_right
                                } else {
                                    false
                                }
                            } else { false }
                        } else { false }
                    }
                    else if Regex::new("<[0-9]+").unwrap().is_match(pattern) {
                        let p = pattern.replace("<", "");

                        if let Ok(p) = p.parse::<usize>() {
                            if let Ok(s) = target.parse::<usize>() {
                                s <= p
                            } else { false }
                        } else { false }
                    } else if Regex::new(">[0-9]+").unwrap().is_match(pattern) {

                        let p = pattern.replace(">", "");

                        if let Ok(p) = p.parse::<usize>() {
                            if let Ok(s) = target.parse::<usize>() {
                                s >= p
                            } else { false }
                        } else { false }
                    }
                    else {
                        target.starts_with(pattern)
                    }
                ,
            ),
            cancelled: ColumnFilterBridge::new(
                |flt: &Flight| flt.cancelled,
                |flt| if flt.cancelled { "Yes".to_string() } else { "No".to_string() },
                |pattern, target| target.contains(pattern),
            ),
            gate: ColumnFilterBridge::new(
                |flt: &Flight| flt.gate.clone().unwrap_or("N/A".to_string()),
                |flt| flt.gate.clone().unwrap_or("N/A".to_string()),
                |pattern, target| pattern.split(",").into_iter().any(|d| d == target),
            ),
        }
    }
}
*/



impl Default for TableFilterApp {
    fn default() -> Self {
        let flights = Rc::new(generate_random_flights(1_000));
        let table_filter = Rc::new(TableFilter::new(&flights));

        // ORIG FILTER
        {
            struct OrigColumnFilter {
                column_filter_state: ColumnFilterState<Flight>
            }
            impl ColumnFilter<Flight> for OrigColumnFilter {
                fn id(&self) -> &'static str { "orig_filter" }
                fn get_value(&self, t: &Flight) -> ScalarValue { ScalarValue::Str(t.orig.clone()) }
                fn column_filter_state(&self) -> &ColumnFilterState<Flight> { &self.column_filter_state }
            }

            table_filter.add_column(Box::new(
                OrigColumnFilter { column_filter_state: ColumnFilterState::new(&table_filter) }
            ));
        }

        // DEST FILTER
        {
            struct DestColumnFilter {
                column_filter_state: ColumnFilterState<Flight>
            }
            impl ColumnFilter<Flight> for DestColumnFilter {
                fn id(&self) -> &'static str { "dest_filter" }
                fn get_value(&self, t: &Flight) -> ScalarValue { ScalarValue::Str(t.dest.clone()) }
                fn column_filter_state(&self) -> &ColumnFilterState<Flight> { &self.column_filter_state }
            }

            table_filter.add_column(Box::new(
                DestColumnFilter { column_filter_state: ColumnFilterState::new(&table_filter) }
            ));
        }

        Self {
            flights, // TODO: explore performance concerns for larger number of records
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
/*                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::remainder())*/
                .header(20.0, |mut header| {


                    // ORIG COLUMN

                    let (_, orig_resp) = header.col(|ui| {
                        ui.strong("ORIG");
                        if self.table_filter.is_active_for_id("orig_filter") {
                            ui.strong("🌰");
                        }
                    });
                    self.table_filter.bind_for_id("orig_filter", orig_resp);


                    // DEST COLUMN
                    let (_, dest_resp) = header.col(|ui| {
                        ui.strong("DEST");
                        if self.table_filter.is_active_for_id("dest_filter") {
                            ui.strong("🌰");
                        }
                    });
                    self.table_filter.bind_for_id("dest_filter", dest_resp);

/*
                    let (_, dep_date_resp) = header.col(|ui| {
                        ui.strong("DEP DATE");
                        if self.column_filters.dep_date.is_active() {
                            ui.strong("🌰");
                        }
                    });
                    self.column_filters.dep_date.bind(
                        Id::new("dep_date_filter"),
                        dep_date_resp,
                        &self.flights,
                        &self.column_filters.evaluate_array(&self.flights, Some(2))
                    );

                    let (_, mileage_resp) = header.col(|ui| {
                        ui.strong("MILEAGE");
                        if self.column_filters.mileage.is_active() {
                            ui.strong("🌰");
                        }
                    });
                    self.column_filters.mileage.bind(
                        Id::new("mileage_filter"),
                        mileage_resp,
                        &self.flights,
                        &self.column_filters.evaluate_array(&self.flights, Some(3))
                    );

                    let (_, cancelled_resp) = header.col(|ui| {
                        ui.strong("CANCELLED");
                        if self.column_filters.cancelled.is_active() {
                            ui.strong("🌰");
                        }
                    });
                    self.column_filters.cancelled.bind(
                        Id::new("cancelled_filter"),
                        cancelled_resp,
                        &self.flights,
                        &self.column_filters.evaluate_array(&self.flights, Some(4))
                    );

                    let (_, gate_resp) = header.col(|ui| {
                        ui.strong("GATE NO.");
                        if self.column_filters.gate.is_active() {
                            ui.strong("🌰");
                        }
                    });
                    self.column_filters.gate.bind(
                        Id::new("gate_filter"),
                        gate_resp,
                        &self.flights,
                        &self.column_filters.evaluate_array(&self.flights, Some(5))
                    );*/
                })
                .body(|mut body| {
                    self.flights
                        .iter()
                        .filter(|flt| self.table_filter.evaluate(&flt))
                        .for_each(| flight| {
                        body.row(30.0, |mut row| {
                            row.col(|ui| {
                                ui.label(&flight.orig);
                            });
                            row.col(|ui| {
                                ui.label(&flight.dest);
                            });
/*                            row.col(|ui| {
                                ui.label(flight.dep_date.format("%m/%d/%Y").to_string());
                            });
                            row.col(|ui| {
                                ui.label(flight.mileage.to_string());
                            });
                            row.col(|ui| {
                                ui.checkbox(&mut flight.cancelled, "");
                            });
                            row.col(|ui| {
                                let mut option_proxy = flight.gate.clone().unwrap_or(String::default());
                                if ui.text_edit_singleline(&mut option_proxy).changed() {
                                    flight.gate = if option_proxy.is_empty() { None } else { Some(option_proxy) };
                                }
                            });*/
                        });
                    });
                });
        });
    }
}