use std::any::Any;
use std::collections::HashSet;
use std::error::Error;
use eframe::egui;
use eframe::App;
use chrono::NaiveDate;
use egui::{Id, Label, Sense};
use egui_extras::{TableBuilder, Column};
use itertools::Itertools;
use regex::Regex;
use crate::data::generate_random_flights;
use crate::table_filter::{ColumnFilter, TableFilter};

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
    flights: Vec<Flight>,
    column_filters: ColumnFilters,
}

struct ColumnFilters {
    orig: ColumnFilter<Flight, String>,
    dest: ColumnFilter<Flight, String>,
    dep_date: ColumnFilter<Flight, String>,
    mileage: ColumnFilter<Flight, u32>,
    cancelled: ColumnFilter<Flight, bool>,
    gate: ColumnFilter<Flight, String>,
}

impl TableFilter<Flight> for ColumnFilters {
    fn check_for_reset(&mut self) {
        if self.orig.reset_called() ||
            self.dest.reset_called() ||
            self.mileage.reset_called() ||
            self.dep_date.reset_called() ||
            self.cancelled.reset_called() ||
            self.gate.reset_called() {

            self.orig.reset();
            self.dest.reset();
            self.mileage.reset();
            self.dep_date.reset();
            self.cancelled.reset();
            self.gate.reset();
        }
    }
    fn evaluate(&self, flight: &Flight) -> bool {
        self.orig.evaluate(&flight) &&
            self.dest.evaluate(&flight) &&
            self.dep_date.evaluate(&flight) &&
            self.mileage.evaluate(&flight) &&
            self.cancelled.evaluate(&flight) &&
            self.gate.evaluate(&flight)
    }
    fn evaluate_array(&self, flights: &Vec<Flight>, exclude_idx: Option<usize>) -> Vec<bool> {
        let evals = [
            self.orig.get_eval_bool_array(&flights),
                self.dest.get_eval_bool_array(&flights),
                self.dep_date.get_eval_bool_array(&flights),
                self.mileage.get_eval_bool_array(&flights),
                self.cancelled.get_eval_bool_array(&flights),
                self.gate.get_eval_bool_array(&flights)
        ];

        assert!(!evals.is_empty());
        let len = evals[0].len();
        // Defensive check: ensure all have same length
        assert!(evals.iter().all(|v| v.len() == len));

        let mut result = vec![true; len]; // Start with all true
        for (i, eval) in evals.iter().enumerate() {
            if let Some(j) = exclude_idx {
                if i == j {
                    continue;
                }
            }
            for (r, &b) in result.iter_mut().zip(eval.iter()) {
                *r &= b;
            }
        }
        result
    }
}

impl Default for ColumnFilters {
    fn default() -> Self {
        Self {
            orig: ColumnFilter::new(
                |flt: &Flight| flt.orig.clone(),
                |flt| flt.orig.clone(),
                |pattern, target| target.starts_with(pattern),
            ),
            dest: ColumnFilter::new(
                |flt: &Flight| flt.dest.clone(),
                |flt| flt.dest.clone(),
                |pattern, target| target.starts_with(pattern),
            ),
            dep_date: ColumnFilter::new(
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
            mileage: ColumnFilter::new(
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
            cancelled: ColumnFilter::new(
                |flt: &Flight| flt.cancelled,
                |flt| if flt.cancelled { "Yes".to_string() } else { "No".to_string() },
                |pattern, target| target.contains(pattern),
            ),
            gate: ColumnFilter::new(
                |flt: &Flight| flt.gate.clone().unwrap_or("N/A".to_string()),
                |flt| flt.gate.clone().unwrap_or("N/A".to_string()),
                |pattern, target| pattern.split(",").into_iter().any(|d| d == target),
            ),
        }
    }
}

impl Default for TableFilterApp {
    fn default() -> Self {
        Self {
            flights: generate_random_flights(1_000), // TODO: explore performance concerns for larger number of records
            column_filters: Default::default(),
        }
    }
}

impl App for TableFilterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Flights");

            ui.style_mut().interaction.selectable_labels = false;

            // check for filter reset
            self.column_filters.check_for_reset();

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
                    let (_, orig_resp) = header.col(|ui| {
                        ui.strong("ORIG");
                        if self.column_filters.orig.is_active() {
                            ui.strong("🌰");
                        }
                    });
                    self.column_filters.orig.bind(
                        Id::new("orig_filter"),
                        orig_resp,
                        &self.flights,
                        &self.column_filters.evaluate_array(&self.flights, Some(0))
                    );

                    let (_, dest_resp) = header.col(|ui| {
                        ui.strong("DEST");
                        if self.column_filters.dest.is_active() {
                            ui.strong("🌰");
                        }
                    });
                    self.column_filters.dest.bind(
                        Id::new("dest_filter"),
                        dest_resp,
                        &self.flights,
                        &self.column_filters.evaluate_array(&self.flights,Some(1))
                    );

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
                    );
                })
                .body(|mut body| {
                    self.flights
                        .iter_mut()
                        .filter(|flt| self.column_filters.evaluate(&flt))
                        .for_each(| flight| {
                        body.row(30.0, |mut row| {
                            row.col(|ui| {
                                ui.label(&flight.orig);
                            });
                            row.col(|ui| {
                                ui.label(&flight.dest);
                            });
                            row.col(|ui| {
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
                            });
                        });
                    });
                });
        });
    }
}