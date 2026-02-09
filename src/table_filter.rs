    use std::cell::RefCell;
    use std::collections::{HashSet};
    use std::hash::Hash;
    use std::iter::zip;
    use std::rc::Rc;
    use eframe::emath::RectAlign;
    use egui::{ScrollArea, Id, Popup, PopupCloseBehavior, Response, TextEdit, RichText, Color32};
    use itertools::Itertools;

    pub struct TableFilter<T> {
        backing_data: Rc<Vec<T>>,
        column_filters: RefCell<Vec<Box<dyn ColumnFilter<T>>>>
    }

    impl <T> TableFilter<T> {
        pub fn new(backing_data: &Rc<Vec<T>>) -> Rc<Self> {
            Rc::new(
                Self {
                    backing_data: Rc::clone(backing_data),
                    column_filters: RefCell::new(vec![])
                }
            )
        }

        pub fn evaluate(&self, item: &T) -> bool {
            self.column_filters.borrow().iter().all(|cf| cf.evaluate(item))
        }
        pub fn reset(&self) {
            self.column_filters.borrow().iter().for_each(|cf| cf.reset());
        }

        pub fn column_filter(&self, cf: Box<dyn ColumnFilter<T>>) {
            self.column_filters.borrow_mut().push(cf);
        }

        pub fn is_active_for_id(&self, id: &str) -> bool {
            self.column_filters.borrow().iter()
                .filter(|cf| *cf.id() == *id)
                .any(|cf| cf.is_active())
        }
        pub fn bind_for_id(&self, id: &str, response: Response) {
            self.column_filters.borrow().iter()
                .find(|cf| *cf.id() == *id)
                .map(|cf| {
                    cf.bind(response);
                });
        }
    }

    pub struct ColumnFilterState<T> {
        table_filter: Rc<TableFilter<T>>,
        unselected_values: RefCell<HashSet<ScalarValue>>,
        search_field: RefCell<String>
    }
    impl <T> ColumnFilterState<T> {
        pub fn new(table_filter: &Rc<TableFilter<T>>) -> Self {
            Self {
                table_filter: Rc::clone(table_filter),
                unselected_values: RefCell::new(Default::default()),
                search_field: RefCell::new("".to_string()),
            }
        }
    }

    #[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum ScalarValue {
        Str(String),
        U32(u32),
        I32(i32),
        Bool(bool),
    }
    impl std::fmt::Display for ScalarValue {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ScalarValue::Str(s) => write!(f, "{}", s),
                ScalarValue::U32(u) => write!(f, "{}", u),
                ScalarValue::Bool(b) => write!(f, "{}", b),
                ScalarValue::I32(i) => write!(f, "{}", i),
            }
        }
    }


    pub trait ColumnFilter<T> {
        fn id(&self) -> &str;
        fn get_value(&self, t: &T) -> ScalarValue;

        fn column_filter_state(&self) -> &ColumnFilterState<T>;

        // default implementations
        fn get_eval_bool_array(&self) -> Vec<bool> {
            self.column_filter_state().table_filter.backing_data
                .iter()
                .map(|t| self.evaluate(t))
                .collect()
        }
        fn selectable_value_bool_array(&self) -> Vec<bool> {
            let evals = self.column_filter_state().table_filter
                .column_filters.borrow().iter()
                .filter(|cf| cf.id() != self.id())
                .map(|cf| cf.get_eval_bool_array())
                .collect::<Vec<_>>();

            assert!(!evals.is_empty());
            let len = evals[0].len();
            // Defensive check: ensure all have same length
            assert!(evals.iter().all(|v| v.len() == len));

            let mut result = vec![true; len]; // Start with all true
            for eval in evals {
                for (r, &b) in result.iter_mut().zip(eval.iter()) {
                    *r &= b;
                }
            }
            result
        }

        fn reset(&self) {
            self.column_filter_state().search_field.borrow_mut().clear();
            self.column_filter_state().unselected_values.borrow_mut().clear();
        }

        fn contains(&self, value: &ScalarValue) -> bool {
            !self.column_filter_state().unselected_values.borrow().contains(value)
        }
        fn search_pattern(&self, target: &String, pattern: &String) -> bool {
            target.starts_with(pattern)
        }
        fn get_string_value(&self, t: &T) -> String {
            self.get_value(t).to_string()
        }
        fn evaluate(&self, t: &T) -> bool {
            let v = self.get_value(t);
            !self.column_filter_state().unselected_values.borrow().contains(&v)
        }
        fn is_active(&self) -> bool {
            !self.column_filter_state().unselected_values.borrow().is_empty()
        }
        fn bind(&self, response: Response)  {
            // add popup
            Popup::menu(&response).id(Id::new(self.id()))
                .align(RectAlign::default())
                .gap(4.0)
                .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
                .width(150.0)
                .show(|ui| {
                    ui.vertical(|ui| {

                        ui.label("Search...");

                        {
                            let mut search_field = self.column_filter_state().search_field.borrow_mut();

                            let search_input = TextEdit::singleline(&mut *search_field)
                                .desired_width(ui.available_width());

                            ui.add(search_input);
                        }

/*                        if ui.input(|input| input.key_pressed(Key::Enter)) {
                            *self.apply_button_clicked() = true;
                        }*/

                        let filter_array = self.selectable_value_bool_array();

                        // selectable values
                        ScrollArea::vertical()
                            .min_scrolled_height(300.0)
                            .max_height(300.0)
                            .show(ui, |ui| {

                                ui.vertical(|ui| {

                                    let search_field_empty = self.column_filter_state().search_field.borrow().is_empty();

                                    let visible_unique: HashSet<ScalarValue> = zip(self.column_filter_state().table_filter.backing_data.iter(), filter_array)
                                        .map(|(d, b)| (self.get_value(&d),b))
                                        .filter(|(d,b)| *b)
                                        .map(|(d,b)| d)
                                        .collect();


                                    self.column_filter_state().table_filter.backing_data.iter()
                                        .filter(|d|search_field_empty ||
                                            self.search_pattern(&self.column_filter_state().search_field.borrow(), &self.get_string_value(d))
                                        )
                                        .unique_by(|d| self.get_value(d))
                                        .sorted_by_key(|d| self.get_value(d))
                                        .for_each(|d| {
                                            let v = self.get_value(d);

                                            let label = if !visible_unique.contains(&v) {
                                                RichText::new(&self.get_string_value(d)).weak()
                                            } else {
                                                RichText::new(&self.get_string_value(d))
                                            };

                                            let mut checked = !self.column_filter_state().unselected_values.borrow().contains(&v) && (
                                                self.column_filter_state().search_field.borrow().is_empty() ||
                                                    self.search_pattern(&self.column_filter_state().search_field.borrow(), &self.get_string_value(d))
                                            );

                                            if ui.checkbox(&mut checked, label).clicked() {
                                                if checked {
                                                    self.column_filter_state().unselected_values.borrow_mut().remove(&v);
                                                } else {
                                                    self.column_filter_state().unselected_values.borrow_mut().insert(v);
                                                }
                                            }
                                        });
                                });
                            });
                        ui.add_space(20.0);

                        ui.horizontal(|ui| {
                            if ui.button("APPLY").clicked() {
                                if !self.column_filter_state().search_field.borrow().is_empty() {
                                    self.column_filter_state().table_filter.backing_data.iter()
                                        .unique_by(|d| self.get_value(d))
                                        .collect::<Vec<_>>()
                                        .iter()
                                        .for_each(|d| {
                                            let v = self.get_value(&d);
                                            if self.search_pattern(&self.column_filter_state().search_field.borrow(), &self.get_string_value(d)) {
                                                self.column_filter_state().unselected_values.borrow_mut().remove(&v);
                                            } else {
                                                self.column_filter_state().unselected_values.borrow_mut().insert(v);
                                            }
                                        });
                                }
                                ui.close();
                            }

                            if ui.button("NONE").clicked() {
                                self.column_filter_state().table_filter.backing_data.iter()
                                    .unique_by(|d| self.get_value(d))
                                    .collect::<Vec<_>>()
                                    .iter()
                                    .for_each(|d| {
                                        let v = self.get_value(&d);
                                        self.column_filter_state().unselected_values.borrow_mut().insert(v);
                                    });
                            }


                            if ui.button("ALL").clicked() {
                                self.column_filter_state().table_filter.backing_data.iter()
                                    .unique_by(|d| self.get_value(d))
                                    .collect::<Vec<_>>()
                                    .iter()
                                    .for_each(|d| {
                                        let v = self.get_value(&d);
                                        self.column_filter_state().unselected_values.borrow_mut().remove(&v);
                                    });
                            }

                            if ui.button("RESET").clicked() {
                                self.column_filter_state().table_filter.reset();
                                ui.close();
                            }
                        });
                    });
                });
        }
    }
