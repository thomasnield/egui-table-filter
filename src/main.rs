use std::any::Any;
use eframe::egui;
use eframe::App;
use chrono::NaiveDate;
use egui::{Id, Label, Sense};
use egui_extras::{TableBuilder, Column};
use crate::table_filter::ColumnFilter;

mod table_filter;

#[derive(Clone)]
struct Flight {
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
    dep_date: ColumnFilter<Flight, NaiveDate>,
    mileage: ColumnFilter<Flight, u32>,
    cancelled: ColumnFilter<Flight, bool>,
    gate: ColumnFilter<Flight, String>,
}

impl ColumnFilters {
    pub fn check_for_reset(&mut self) {
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
    pub fn evaluate(&self, flight: &Flight) -> bool {
        self.orig.evaluate(&flight) &&
            self.dest.evaluate(&flight) &&
            self.mileage.evaluate(&flight) &&
            self.dep_date.evaluate(&flight) &&
            self.cancelled.evaluate(&flight) &&
            self.gate.evaluate(&flight)
    }
}

impl Default for ColumnFilters {
    fn default() -> Self {
        Self {
            orig: ColumnFilter::new(
                |flt: &Flight| flt.orig.clone(),
                |flt| flt.orig.clone(),
                |pattern, target| target.contains(pattern),
            ),
            dest: ColumnFilter::new(
                |flt: &Flight| flt.dest.clone(),
                |flt| flt.dest.clone(),
                |pattern, target| target.contains(pattern),
            ),
            dep_date: ColumnFilter::new(
                |flt: &Flight| flt.dep_date,
                |flt| flt.dep_date.format("%Y-%m-%d").to_string(),
                |pattern, target| target.contains(pattern),
            ),
            mileage: ColumnFilter::new(
                |flt: &Flight| flt.mileage,
                |flt| flt.mileage.to_string(),
                |pattern, target| target.contains(pattern),
            ),
            cancelled: ColumnFilter::new(
                |flt: &Flight| flt.cancelled,
                |flt| if flt.cancelled { "Yes".to_string() } else { "No".to_string() },
                |pattern, target| target.contains(pattern),
            ),
            gate: ColumnFilter::new(
                |flt: &Flight| flt.gate.clone().unwrap_or("N/A".to_string()),
                |flt| flt.gate.clone().unwrap_or("N/A".to_string()),
                |pattern, target| target.contains(pattern),
            ),
        }
    }
}

impl Default for TableFilterApp {
    fn default() -> Self {
        Self {
            flights: vec![
                Flight { number: 567, orig: "ABQ".to_string(), dest: "DAL".to_string(), dep_date: NaiveDate::from_ymd_opt(2015, 5, 8).unwrap(), mileage: 642, cancelled: false, gate: Some("23".to_string()) },
                Flight { number: 234, orig: "ABQ".to_string(), dest: "DAL".to_string(), dep_date: NaiveDate::from_ymd_opt(2015, 5, 9).unwrap(), mileage: 642, cancelled: false, gate: Some("13".to_string()) },
                Flight { number: 756, orig: "ABQ".to_string(), dest: "DAL".to_string(), dep_date: NaiveDate::from_ymd_opt(2015, 5, 11).unwrap(), mileage: 642, cancelled: false, gate: Some("9".to_string()) },
                Flight { number: 268, orig: "ABQ".to_string(), dest: "DAL".to_string(), dep_date: NaiveDate::from_ymd_opt(2015, 5, 13).unwrap(), mileage: 642, cancelled: false, gate: Some("2".to_string()) },
                Flight { number: 567, orig: "DAL".to_string(), dest: "HOU".to_string(), dep_date: NaiveDate::from_ymd_opt(2015, 5, 8).unwrap(), mileage: 244, cancelled: false, gate: Some("A5".to_string()) },
                Flight { number: 239, orig: "DAL".to_string(), dest: "HOU".to_string(), dep_date: NaiveDate::from_ymd_opt(2015, 5, 14).unwrap(), mileage: 244, cancelled: false, gate: Some("B4".to_string()) },
                Flight { number: 5923, orig: "DAL".to_string(), dest: "HOU".to_string(), dep_date: NaiveDate::from_ymd_opt(2015, 5, 17).unwrap(), mileage: 244, cancelled: false, gate: Some("C3".to_string()) },
                Flight { number: 2389, orig: "DAL".to_string(), dest: "HOU".to_string(), dep_date: NaiveDate::from_ymd_opt(2015, 5, 6).unwrap(), mileage: 244, cancelled: false, gate: None },
                Flight { number: 287, orig: "SEA".to_string(), dest: "PHX".to_string(), dep_date: NaiveDate::from_ymd_opt(2015, 5, 8).unwrap(), mileage: 1100, cancelled: false, gate: None },
                Flight { number: 875, orig: "SEA".to_string(), dest: "PHX".to_string(), dep_date: NaiveDate::from_ymd_opt(2015, 5, 16).unwrap(), mileage: 1100, cancelled: false, gate: Some("12".to_string()) },
                Flight { number: 4288, orig: "SEA".to_string(), dest: "PHX".to_string(), dep_date: NaiveDate::from_ymd_opt(2015, 5, 9).unwrap(), mileage: 1100, cancelled: false, gate: Some("19".to_string()) },
                Flight { number: 567, orig: "BWI".to_string(), dest: "MCO".to_string(), dep_date: NaiveDate::from_ymd_opt(2015, 7, 9).unwrap(), mileage: 898, cancelled: false, gate: Some("45".to_string()) },
                Flight { number: 234, orig: "MDW".to_string(), dest: "PDX".to_string(), dep_date: NaiveDate::from_ymd_opt(2015, 7, 12).unwrap(), mileage: 2118, cancelled: false, gate: Some("B9".to_string()) },
                Flight { number: 411, orig: "SAN".to_string(), dest: "JFK".to_string(), dep_date: NaiveDate::from_ymd_opt(2015, 7, 19).unwrap(), mileage: 2077, cancelled: false, gate: None },
            ],
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
                    );

                    let (_, cancelled_resp) = header.col(|ui| {
                        ui.strong("CANCELLED");
                        ui.strong("MILEAGE");
                        if self.column_filters.cancelled.is_active() {
                            ui.strong("🌰");
                        }
                    });
                    self.column_filters.cancelled.bind(
                        Id::new("cancelled_filter"),
                        cancelled_resp,
                        &self.flights,
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
                                ui.label(flight.dep_date.format("%Y-%m-%d").to_string());
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