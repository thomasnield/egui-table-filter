    use std::collections::{HashSet};
    use std::hash::Hash;
    use std::iter::zip;
    use std::ops::Deref;
    use eframe::emath::RectAlign;
    use egui::{ScrollArea, Id, Popup, PopupCloseBehavior, Response, Key, TextEdit, RichText, Color32};

    use itertools::Itertools;

    pub trait TableFilterImpl<T> {
        fn check_for_reset(&mut self);
        fn evaluate(&self, item: &T) -> bool;
        fn evaluate_array(&self, items: &Vec<T>, exclude_idx: Option<usize>) -> Vec<bool>;
    }

    pub struct TableFilter<T> {
        column_filters: Vec<Box<dyn ColumnFilter<T>>>
    }

    impl <T: 'static> TableFilter<T> {
        pub fn add<V: Eq + Hash + Ord + 'static>(
            &mut self,
            get_value: impl Fn(&T) -> V + 'static,
            get_string_value: impl Fn(&T) -> String + 'static,
            str_search_op: impl  Fn(&String,&String) -> bool + 'static
        ) {
            self.column_filters.push(Box::new(ColumnFilterImpl::new(
                get_value,
                get_string_value,
                str_search_op
            )));
        }
    }
    impl <T> TableFilterImpl<T> for TableFilter<T> {
        fn check_for_reset(&mut self) {
            if self.column_filters.iter().any(|cf| cf.reset_called()) {
                self.column_filters.iter_mut().for_each(|cf| cf.reset());
            }
        }

        fn evaluate(&self, item: &T) -> bool {
            self.column_filters.iter().all(|cf| cf.evaluate(item))
        }

        fn evaluate_array(&self, items: &Vec<T>, exclude_idx: Option<usize>) -> Vec<bool> {
            let evals: Vec<Vec<bool>> = self.column_filters.iter().map(|cf| cf.get_eval_bool_array(items)).collect();

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

    pub trait ColumnFilter<T> {
        fn evaluate(&self, t: &T) -> bool;
        fn get_eval_bool_array(&self, data: &Vec<T>) -> Vec<bool>;
        fn reset(&mut self);
        fn reset_called(&self) -> bool;
    }
    pub struct ColumnFilterImpl<T,V: Eq + Hash + Ord> {
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

    impl <T,V: Eq + Hash + Ord> ColumnFilter<T> for ColumnFilterImpl<T,V> {
        fn evaluate(&self, t: &T) -> bool {
            let f = self.get_value.deref();
            !self.unselected_values.contains(&f(t))
        }
        fn get_eval_bool_array(&self, data: &Vec<T>) -> Vec<bool> {
            data.iter()
                .map(|t| self.evaluate(t))
                .collect()
        }
        fn reset(&mut self) {
            self.search_field.clear();
            self.unselected_values.clear();
            self.reset_button_clicked = false;
        }
        fn reset_called(&self) -> bool {
            self.reset_button_clicked
        }
    }
    impl <T,V: Eq + Hash + Ord> ColumnFilterImpl<T,V> {
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

    impl <T,V: Eq + Hash + Ord + std::fmt::Display> ColumnFilterImpl<T,V> {


        pub fn contains(&self, value: &V) -> bool {
            !self.unselected_values.contains(value)
        }


        pub fn bind(&mut self, id: Id,
                    response: Response,
                    data: &Vec<T>,
                    filter_array: Vec<bool>
        )  {

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

                                    let visible_unique: HashSet<V> = zip(data, filter_array)
                                        .map(|(d, b)| (self.get_value(d),b))
                                        .filter(|(d,b)| *b)
                                        .map(|(d,b)| d)
                                        .collect();

                                    data.iter()
                                        .filter(|d| self.search_field.is_empty() ||  self.search_pattern(&self.search_field, &self.get_string_value(d)))
                                        .unique_by(|d| self.get_value(d))
                                        .sorted_by_key(|d| self.get_value(d))
                                        .for_each(|d| {
                                            let v = self.get_value(d);

                                            let label = if !visible_unique.contains(&v) {
                                                RichText::new(&self.get_string_value(d)).weak()
                                            } else {
                                                RichText::new(&self.get_string_value(d))
                                            };

                                            let mut checked = !self.unselected_values.contains(&v) && (
                                                self.search_field.is_empty() || self.search_pattern(&self.search_field, &self.get_string_value(d))
                                            );

                                            if ui.checkbox(&mut checked, label).clicked() {
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