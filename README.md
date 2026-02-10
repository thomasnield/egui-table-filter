This is my successful port of a table filter control in Rust for the egui library. It's approaching something that I'd be happy to publish as a crate. 

I first made this for [JavaFX almost 10 years ago](http://fxexperience.com/2016/03/introducing-the-controlsfx-tablefilter/). 

It can be used alongside the `TableBuilder` in [egui-extras](https://docs.rs/egui_extras/latest/egui_extras/), but I suspect it can be adapted to other data controls. 

I would like to make this into a library, so I welcome any contributions to make the API streamlined.

<img width="640" height="546" alt="image" src="https://github.com/user-attachments/assets/2a8a7bbf-5757-4758-95d6-5e2b73aa29a8" />

You can also create custom search functionality to parse the strings for special syntax, like regular expressions or date ranges. 

<img width="586" height="593" alt="image" src="https://github.com/user-attachments/assets/3e6a93cc-b93d-4841-97f5-41d20eb31ed4" />

[VIDEO DEMO](https://www.youtube.com/watch?v=dbkbqy4TxCY) 

## Usage 

In your application state, set up your filters for specific columns you want. The backing list must be an `Rc<Vec<T>>` so there is no contention of ownership, and the `TableFilter` will be in an `Rc` as well. 

```rust 
struct TableFilterApp {
    flights: Rc<Vec<Flight>>,
    table_filter: Rc<TableFilter<Flight>>,
}

impl Default for TableFilterApp {
    fn default() -> Self {
        // backing data and table filter objects MUST be in a Rc.
        let flights = Rc::new(generate_random_flights(10_000));
        let table_filter = TableFilter::new(&flights); // <- returns an Rc<TableFilter>

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
```

And on the Table declaration, use the binding macros on the column headers. 

```rust 
impl App for TableFilterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Flights");

            ui.style_mut().interaction.selectable_labels = false;
            let text_style = egui::TextStyle::Body;
            let row_height = ui.text_style_height(&text_style) + 10.0;

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

                    let filtered_flights = self.flights.iter()
                        .filter(|flt| self.table_filter.evaluate(&flt))
                        .collect::<Vec<_>>();

                    let total_rows = filtered_flights.len();

                    // use rows to only render the rows that are in scrolled view
                    body.rows(row_height, total_rows, |mut row| {
                        let flight = filtered_flights[row.index()];

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
    }
}
```

The structs and traits make this highly extensible. I will document how to do this later but the source code shows many examples. 

## TODO

- [X] Gray out entries that are no longer visible due to other column filter
- [X] Extract out common methods as traits for `ColumnFilters` implementations
- [X] Stress test on a larger dataset
- [X] Find opportunities to make more idiomatic without making overly opinionated (macros might be needed :/)
- [X] Have pre-defined templates to streamline common data types and operations (e.g., search with int ranges, date ranges, regular expressions, comma-separated values)
- [X] Expand custom searchbox logic examples
- [ ] Documentation on usage
