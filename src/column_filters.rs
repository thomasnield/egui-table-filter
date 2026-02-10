use std::cell::{LazyCell};
use std::rc::Rc;
use chrono::NaiveDate;
use itertools::Itertools;
use regex::Regex;
use crate::table_filter::{ColumnFilter, ColumnFilterState, ScalarValue, TableFilter};

pub struct StringColumnFilter<T> {
    id: String,
    column_filter_state: ColumnFilterState<T>,
    mapper: Box<dyn Fn(&T) -> String>
}

impl <T> StringColumnFilter<T> {
    pub fn new(id: &str, table_filter: Rc<TableFilter<T>>, mapper: Box<dyn Fn(&T) -> String>) -> Self {
        Self {
            id: id.to_string(),
            column_filter_state: ColumnFilterState::new(&table_filter),
            mapper
        }
    }
}

impl <T> ColumnFilter<T> for StringColumnFilter<T> {
    fn id(&self) -> &str { self.id.as_str() }
    fn get_value(&self, t: &T) -> ScalarValue { ScalarValue::Str((self.mapper)(t)) }
    fn column_filter_state(&self) -> &ColumnFilterState<T> { &self.column_filter_state }
    fn search_pattern(&self, pattern: &String, target: &String) -> bool {
        // search for multiple values separated by commas
        pattern.split(",").any(|pattern| {
            target.starts_with(pattern)
        })
    }
}

#[macro_export]
macro_rules! string_filters {
    // This pattern allows: string_filters!(table, ("id1", |x| ...), ("id2", |x| ...))
    ($table:expr, $( ($id:expr, |$arg:ident| $mapper:expr) ),* $(,)?) => {
        $(
            $table.column_filter(Box::new(
                $crate::StringColumnFilter::new(
                    $id,
                    std::rc::Rc::clone(&$table),
                    Box::new(|$arg| $mapper)
                )
            ));
        )*
    };
}

pub struct U32ColumnFilter<T> {
    id: String,
    column_filter_state: ColumnFilterState<T>,
    mapper: Box<dyn Fn(&T) -> u32>,
    str_mapper: Box<dyn Fn(&T) -> String>
}

impl <T> U32ColumnFilter<T> {
    pub fn new(id: &str, table_filter: Rc<TableFilter<T>>, mapper: Box<dyn Fn(&T) -> u32>, str_mapper: Box<dyn Fn(&T) -> String>) -> Self {
        Self {
            id: id.to_string(),
            column_filter_state: ColumnFilterState::new(&table_filter),
            mapper,
            str_mapper
        }
    }
    const LESS_THAN_REGEX: LazyCell<Regex> = LazyCell::new(|| Regex::new(r#"^<[0-9]+$"#).unwrap());
    const LESS_THAN_EQUAL_REGEX: LazyCell<Regex> = LazyCell::new(|| Regex::new(r#"^<=[0-9]+$"#).unwrap());
    const GREATER_THAN_REGEX: LazyCell<Regex> = LazyCell::new(|| Regex::new(r#"^>[0-9]+$"#).unwrap());
    const GREATER_THAN_EQUAL_REGEX: LazyCell<Regex> = LazyCell::new(|| Regex::new(r#"^>=[0-9]+$"#).unwrap());
}

impl <T> ColumnFilter<T> for U32ColumnFilter<T> {
    fn id(&self) -> &str { self.id.as_str() }
    fn get_value(&self, t: &T) -> ScalarValue { ScalarValue::U32((self.mapper)(t)) }
    fn get_string_value(&self, t: &T) -> String { (self.str_mapper)(t) }
    fn column_filter_state(&self) -> &ColumnFilterState<T> { &self.column_filter_state }
    fn search_pattern(&self, pattern: &String, target: &String) -> bool {
        pattern.split(",").into_iter().all(|pattern| {
            if pattern.contains("<=") && Self::LESS_THAN_EQUAL_REGEX.is_match(pattern) {
                let x: Result<u32, _> = target.parse();
                let y: Result<u32, _> = pattern.replace("<=", "").parse();
                if let Ok(x) = x && let Ok(y) = y {
                    x <= y
                } else {
                    false
                }
            } else if pattern.contains(">=") && Self::GREATER_THAN_EQUAL_REGEX.is_match(pattern) {
                let x: Result<u32, _> = target.parse();
                let y: Result<u32, _> = pattern.replace(">=", "").parse();
                if let Ok(x) = x && let Ok(y) = y {
                    x >= y
                } else {
                    false
                }
            } else if pattern.contains("<") && Self::LESS_THAN_REGEX.is_match(pattern) {
                let x: Result<u32, _> = target.parse();
                let y: Result<u32, _> = pattern.replace("<", "").parse();
                if let Ok(x) = x && let Ok(y) = y {
                    x < y
                } else {
                    false
                }
            } else if pattern.contains(">") && Self::GREATER_THAN_REGEX.is_match(pattern) {
                let x: Result<u32, _> = target.parse();
                let y: Result<u32, _> = pattern.replace(">", "").parse();
                if let Ok(x) = x && let Ok(y) = y {
                    x > y
                } else {
                    false
                }
            } else {
                target.starts_with(pattern)
            }
        })
    }
}

#[macro_export]
macro_rules! u32_filters {
    // This pattern allows: string_filters!(table, ("id1", |x| ...), ("id2", |x| ...))
    ($table:expr, $( ($id:expr, |$arg:ident| $mapper:expr) ),* $(,)?) => {
        $(
            $table.column_filter(Box::new(
                $crate::U32ColumnFilter::new(
                    $id,
                    std::rc::Rc::clone(&$table),
                    Box::new(|$arg| $mapper),
                    Box::new(|$arg| $mapper.to_string())
                )
            ));
        )*
    };

    ($table:expr, $( ($id:expr, |$arg:ident| $mapper:expr, |$str_arg:ident| $str_mapper:expr) ),* $(,)?) => {
        $(
            $table.column_filter(Box::new(
                $crate::U32ColumnFilter::new(
                    $id,
                    std::rc::Rc::clone(&$table),
                    Box::new(|$arg| $mapper),
                    Box::new(|$str_arg| $str_mapper)
                )
            ));
        )*
    };
}

pub struct I32ColumnFilter<T> {
    id: String,
    column_filter_state: ColumnFilterState<T>,
    mapper: Box<dyn Fn(&T) -> i32>,
    str_mapper: Box<dyn Fn(&T) -> String>,
}

impl <T> I32ColumnFilter<T> {
    pub fn new(id: &str, table_filter: Rc<TableFilter<T>>, mapper: Box<dyn Fn(&T) -> i32>, str_mapper: Box<dyn Fn(&T) -> String>) -> Self {
        Self {
            id: id.to_string(),
            column_filter_state: ColumnFilterState::new(&table_filter),
            mapper,
            str_mapper
        }
    }
    const LESS_THAN_REGEX: LazyCell<Regex> = LazyCell::new(|| Regex::new(r#"^<[0-9]+$"#).unwrap());
    const LESS_THAN_EQUAL_REGEX: LazyCell<Regex> = LazyCell::new(|| Regex::new(r#"^<=[0-9]+$"#).unwrap());
    const GREATER_THAN_REGEX: LazyCell<Regex> = LazyCell::new(|| Regex::new(r#"^>[0-9]+$"#).unwrap());
    const GREATER_THAN_EQUAL_REGEX: LazyCell<Regex> = LazyCell::new(|| Regex::new(r#"^>=[0-9]+$"#).unwrap());
}

#[macro_export]
macro_rules! i32_filters {
    // This pattern allows: string_filters!(table, ("id1", |x| ...), ("id2", |x| ...))
    ($table:expr, $( ($id:expr, |$arg:ident| $mapper:expr) ),* $(,)?) => {
        $(
            $table.column_filter(Box::new(
                $crate::I32ColumnFilter::new(
                    $id,
                    std::rc::Rc::clone(&$table),
                    Box::new(|$arg| $mapper.to_string())
                )
            ));
        )*
    };
    ($table:expr, $( ($id:expr, |$arg:ident| $mapper:expr, |$str_arg:ident| $str_mapper:expr) ),* $(,)?) => {
        $(
            $table.column_filter(Box::new(
                $crate::I32ColumnFilter::new(
                    $id,
                    std::rc::Rc::clone(&$table),
                    Box::new(|$arg| $mapper),
                    Box::new(|$str_arg| $str_mapper)
                )
            ));
        )*
    };
}

impl <T> ColumnFilter<T> for I32ColumnFilter<T> {
    fn id(&self) -> &str { self.id.as_str() }
    fn get_value(&self, t: &T) -> ScalarValue { ScalarValue::I32((self.mapper)(t)) }
    fn get_string_value(&self, t: &T) -> String { (self.str_mapper)(t) }
    fn column_filter_state(&self) -> &ColumnFilterState<T> { &self.column_filter_state }

    fn search_pattern(&self, pattern: &String, target: &String) -> bool {
        pattern.split(",").into_iter().all(|pattern| {
            if pattern.contains("<=") && Self::LESS_THAN_EQUAL_REGEX.is_match(pattern) {
                let x: Result<u32, _> = target.parse();
                let y: Result<u32, _> = pattern.replace("<=", "").parse();
                if let Ok(x) = x && let Ok(y) = y {
                    x <= y
                } else {
                    false
                }
            } else if pattern.contains(">=") && Self::GREATER_THAN_EQUAL_REGEX.is_match(pattern) {
                let x: Result<u32, _> = target.parse();
                let y: Result<u32, _> = pattern.replace(">=", "").parse();
                if let Ok(x) = x && let Ok(y) = y {
                    x >= y
                } else {
                    false
                }
            } else if pattern.contains("<") && Self::LESS_THAN_REGEX.is_match(pattern) {
                let x: Result<u32, _> = target.parse();
                let y: Result<u32, _> = pattern.replace("<", "").parse();
                if let Ok(x) = x && let Ok(y) = y {
                    x < y
                } else {
                    false
                }
            } else if pattern.contains(">") && Self::GREATER_THAN_REGEX.is_match(pattern) {
                let x: Result<u32, _> = target.parse();
                let y: Result<u32, _> = pattern.replace(">", "").parse();
                if let Ok(x) = x && let Ok(y) = y {
                    x > y
                } else {
                    false
                }
            } else {
                target.starts_with(pattern)
            }
        })
    }
}


pub struct NaiveDateColumnFilter<T> {
    id: String,
    column_filter_state: ColumnFilterState<T>,
    date_str_pattern: String,
    mapper: Box<dyn Fn(&T) -> NaiveDate>
}

impl <T> NaiveDateColumnFilter<T> {
    pub fn new(id: &str, table_filter: Rc<TableFilter<T>>, date_str_pattern: String, mapper: Box<dyn Fn(&T) -> NaiveDate>) -> Self {
        Self {
            id: id.to_string(),
            column_filter_state: ColumnFilterState::new(&table_filter),
            date_str_pattern,
            mapper
        }
    }
}

impl <T> ColumnFilter<T> for NaiveDateColumnFilter<T> {
    fn id(&self) -> &str { self.id.as_str() }
    fn get_value(&self, t: &T) -> ScalarValue { ScalarValue::I32((self.mapper)(t).to_epoch_days()) }
    fn get_string_value(&self, t: &T) -> String {

        if let ScalarValue::I32(n) = self.get_value(t) &&
            let Some(s) = NaiveDate::from_epoch_days(n).map(|nd| nd.format(&self.date_str_pattern).to_string()) {
                s
            } else {
                "PARSE ERR".to_string()
            }
    }

    fn column_filter_state(&self) -> &ColumnFilterState<T> { &self.column_filter_state }

    fn search_pattern(&self, pattern: &String, target: &String) -> bool {
        pattern.split(",").into_iter().all(|pattern| {
            if pattern.contains("<=") {
                let x: Result<NaiveDate, _> =NaiveDate::parse_from_str(&target, self.date_str_pattern.as_str());
                let y: Result<NaiveDate, _> = NaiveDate::parse_from_str(pattern.replace("<=", "").as_str(), self.date_str_pattern.as_str());
                if let Ok(x) = x && let Ok(y) = y {
                    x <= y
                } else {
                    false
                }
            } else if pattern.contains(">=") {
                let x: Result<NaiveDate, _> = NaiveDate::parse_from_str(&target, self.date_str_pattern.as_str());
                let y: Result<NaiveDate, _> = NaiveDate::parse_from_str(pattern.replace(">=", "").as_str(), self.date_str_pattern.as_str());
                if let Ok(x) = x && let Ok(y) = y {
                    x >= y
                } else {
                    false
                }
            } else if pattern.contains("<") {
                let x: Result<NaiveDate, _> = NaiveDate::parse_from_str(&target, self.date_str_pattern.as_str());
                let y: Result<NaiveDate, _> = NaiveDate::parse_from_str(pattern.replace("<", "").as_str(), self.date_str_pattern.as_str());
                if let Ok(x) = x && let Ok(y) = y {
                    x < y
                } else {
                    false
                }
            } else if pattern.contains(">") {
                let x: Result<NaiveDate, _> = NaiveDate::parse_from_str(&target, self.date_str_pattern.as_str());
                let y: Result<NaiveDate, _> = NaiveDate::parse_from_str(pattern.replace(">", "").as_str(), self.date_str_pattern.as_str());
                if let Ok(x) = x && let Ok(y) = y {
                    x > y
                } else {
                    false
                }
            } else {
                target.starts_with(pattern)
            }
        })
    }
}

#[macro_export]
macro_rules! naive_date_filters {
    // This pattern allows: string_filters!(table, ("id1", |x| ...), ("id2", |x| ...))
    ($table:expr, $( ($id:expr, |$arg:ident| $mapper:expr) ),* $(,)?) => {
        $(
            $table.column_filter(Box::new(
                $crate::NaiveDateColumnFilter::new(
                    $id,
                    std::rc::Rc::clone(&$table),
                    "%-m/%-d/%Y".to_string(),
                    Box::new(|$arg| $mapper)
                )
            ));
        )*
    };
    ($table:expr, $( ($id:expr, |$arg:ident| $mapper:expr, $formatter:expr) ),* $(,)?) => {
        $(
            $table.column_filter(Box::new(
                $crate::NaiveDateColumnFilter::new(
                    $id,
                    std::rc::Rc::clone(&$table),
                    $formatter.to_string(),
                    Box::new(|$arg| $mapper),
                )
            ));
        )*
    };
}

pub struct BoolColumnFilter<T> {
    id: String,
    column_filter_state: ColumnFilterState<T>,
    mapper: Box<dyn Fn(&T) -> bool>,
    str_mapper: Box<dyn Fn(&T) -> String>,
}

impl <T> BoolColumnFilter<T> {
    pub fn new(id: &str, table_filter: Rc<TableFilter<T>>, mapper: Box<dyn Fn(&T) -> bool>, str_mapper: Box<dyn Fn(&T) -> String>) -> Self {
        Self {
            id: id.to_string(),
            column_filter_state: ColumnFilterState::new(&table_filter),
            mapper,
            str_mapper
        }
    }
}

impl <T> ColumnFilter<T> for BoolColumnFilter<T> {
    fn id(&self) -> &str { self.id.as_str() }
    fn get_value(&self, t: &T) -> ScalarValue { ScalarValue::Bool((self.mapper)(t)) }
    fn get_string_value(&self, t: &T) -> String { (self.str_mapper)(t) }
    fn column_filter_state(&self) -> &ColumnFilterState<T> { &self.column_filter_state }
}

#[macro_export]
macro_rules! bool_filters {
    ($table:expr, $( ($id:expr, |$arg:ident| $mapper:expr) ),* $(,)?) => {
        $(
            $table.column_filter(Box::new(
                $crate::BoolColumnFilter::new(
                    $id,
                    std::rc::Rc::clone(&$table),
                    Box::new(|$arg| $mapper),
                    Box::new(|$arg| $mapper.to_string())
                )
            ));
        )*
    };
    ($table:expr, $( ($id:expr, |$arg:ident| $mapper:expr, |$str_arg:ident| $str_mapper:expr) ),* $(,)?) => {
        $(
            $table.column_filter(Box::new(
                $crate::BoolColumnFilter::new(
                    $id,
                    std::rc::Rc::clone(&$table),
                    Box::new(|$arg| $mapper),
                    Box::new(|$str_arg| $str_mapper)
                )
            ));
        )*
    };
}


#[macro_export]
macro_rules! col_with_filter {
    ($header:expr, $table_filter:expr, $id:expr, |$ui:ident| $body:expr) => {{
        let (_, resp) = $header.col(|$ui| {
            $body
        });
        $table_filter.bind_for_id($id, resp);
    }};
}