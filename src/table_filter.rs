    use std::collections::{HashSet};
    use std::hash::Hash;
    use std::ops::Deref;
    use eframe::emath::RectAlign;
    use egui::{ScrollArea, Id, Popup, PopupCloseBehavior, Response, Key, TextEdit};
    use itertools::Itertools;

    pub struct ColumnFilter<T,V: Eq + Hash + Ord> {
        search_field: String,
        unselected_values: HashSet<V>,
        get_value: Box<dyn Fn(&T) -> V>,
        get_string_value: Box<dyn Fn(&T) -> String>,
        str_search_op: Box<dyn Fn(&String,&String) -> bool>,

        apply_button_clicked: bool,
        reset_button_clicked: bool,
        select_all_clicked: bool,
        select_none_clicked: bool
    }

    impl <T,V: Eq + Hash + Ord> ColumnFilter<T,V> {
        pub fn new(get_value: impl Fn(&T) -> V + 'static,
                   get_string_value: impl Fn(&T) -> String + 'static,
                   str_search_op: impl  Fn(&String,&String) -> bool + 'static) -> Self {
            Self {
                search_field: String::default(),
                unselected_values: HashSet::default(),
                get_value: Box::new(get_value),
                get_string_value: Box::new(get_string_value),
                str_search_op: Box::new(str_search_op),
                select_all_clicked: false,
                select_none_clicked: false,
                apply_button_clicked: false,
                reset_button_clicked: false
            }
        }
        pub fn get_value(&self, t: &T) -> V {
            (self.get_value)(t)
        }
        pub fn get_string_value(&self, t: &T) -> String {
            (self.get_string_value)(t)
        }
        pub fn search_pattern(&self, pattern: &String, string: &String) -> bool {
            (self.str_search_op)(pattern, string)
        }
        pub fn is_active(&self) -> bool {
            !self.unselected_values.is_empty()
        }
    }

    impl <T,V: Eq + Hash + Ord + std::fmt::Display> ColumnFilter<T,V> {

        pub fn evaluate(&self, t: &T) -> bool {
            let f = self.get_value.deref();
            !self.unselected_values.contains(&f(t))
        }
        pub fn contains(&self, value: &V) -> bool {
            !self.unselected_values.contains(value)
        }
        pub fn reset_called(&self) -> bool {
            self.reset_button_clicked
        }
        pub fn reset(&mut self) {
            self.search_field.clear();
            self.unselected_values.clear();
            self.reset_button_clicked = false;
        }

        pub fn bind(&mut self, id: Id,
                    response: Response,
                    data: &Vec<T>)  {

            // add popup
            Popup::menu(&response).id(id)
                .align(RectAlign::default())
                .gap(4.0)
                .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
                .width(150.0)
                .show(|ui| {
                    ui.vertical(|ui| {

                        ui.label("Search...");

                        let search_input = TextEdit::singleline(&mut self.search_field)
                            .desired_width(ui.available_width());

                        ui.add(search_input);

                        if ui.input(|input| input.key_pressed(Key::Enter)) {
                            self.apply_button_clicked = true;
                        }

                        // selectable values
                        ScrollArea::vertical()
                            .min_scrolled_height(300.0)
                            .max_height(300.0)
                            .show(ui, |ui| {

                                ui.vertical(|ui| {

                                    data.iter()
                                        .filter(|d| self.search_field.is_empty() ||  self.search_pattern(&self.search_field, &self.get_string_value(d)))
                                        .unique_by(|d| self.get_value(d))
                                        .sorted_by_key(|d| self.get_value(d))
                                        .for_each(|d| {
                                            let v = self.get_value(d);
                                            let s = self.get_string_value(d);

                                            let mut checked = !self.unselected_values.contains(&v) && (
                                                self.search_field.is_empty() || self.search_pattern(&self.search_field, &self.get_string_value(d))
                                            );

                                            if ui.checkbox(&mut checked, s).clicked() {
                                                if checked {
                                                    self.unselected_values.remove(&v);
                                                } else {
                                                    self.unselected_values.insert(v);
                                                }
                                            }
                                        });
                                });
                            });
                        ui.add_space(20.0);

                        ui.horizontal(|ui| {
                            if ui.button("APPLY").clicked() {
                                self.apply_button_clicked = true;
                            }

                            if self.apply_button_clicked {

                                if !self.search_field.is_empty() {
                                    data.iter()
                                        .unique_by(|d| self.get_value(d))
                                        .collect::<Vec<_>>()
                                        .iter()
                                        .for_each(|d| {
                                            let v = self.get_value(&d);
                                            if self.search_pattern(&self.search_field, &self.get_string_value(d)) {
                                                self.unselected_values.remove(&v);
                                            } else {
                                                self.unselected_values.insert(v);
                                            }
                                        });
                                }
                                self.apply_button_clicked = false;
                                ui.close();
                            }

                            if ui.button("NONE").clicked() {
                                self.select_none_clicked = true;
                            }

                            if self.select_none_clicked {
                                data.iter()
                                    .unique_by(|d| self.get_value(d))
                                    .collect::<Vec<_>>()
                                    .iter()
                                    .for_each(|d| {
                                        let v = self.get_value(&d);
                                        self.unselected_values.insert(v);
                                    });
                                self.select_none_clicked = false;
                            }

                            if ui.button("ALL").clicked() {
                                self.select_all_clicked = true;
                            }
                            if self.select_all_clicked {
                                data.iter()
                                    .unique_by(|d| self.get_value(d))
                                    .collect::<Vec<_>>()
                                    .iter()
                                    .for_each(|d| {
                                        let v = self.get_value(&d);
                                        self.unselected_values.remove(&v);
                                    });
                                self.select_all_clicked = false;
                            }

                            if ui.button("RESET").clicked() {
                                self.reset_button_clicked = true;
                                ui.close();
                            }
                        });
                    });
                });
        }
    }