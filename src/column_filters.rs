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
    mapper: Box<dyn Fn(&T) -> u32>
}

impl <T> U32ColumnFilter<T> {
    pub fn new(id: &str, table_filter: Rc<TableFilter<T>>, mapper: Box<dyn Fn(&T) -> u32>) -> Self {
        Self {
            id: id.to_string(),
            column_filter_state: ColumnFilterState::new(&table_filter),
            mapper
        }
    }
}

impl <T> ColumnFilter<T> for U32ColumnFilter<T> {
    fn id(&self) -> &str { self.id.as_str() }
    fn get_value(&self, t: &T) -> ScalarValue { ScalarValue::U32((self.mapper)(t)) }
    fn column_filter_state(&self) -> &ColumnFilterState<T> { &self.column_filter_state }
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
                    Box::new(|$arg| $mapper)
                )
            ));
        )*
    };
}

pub struct I32ColumnFilter<T> {
    id: String,
    column_filter_state: ColumnFilterState<T>,
    mapper: Box<dyn Fn(&T) -> i32>
}

impl <T> I32ColumnFilter<T> {
    pub fn new(id: &str, table_filter: Rc<TableFilter<T>>, mapper: Box<dyn Fn(&T) -> i32>) -> Self {
        Self {
            id: id.to_string(),
            column_filter_state: ColumnFilterState::new(&table_filter),
            mapper
        }
    }
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
                    Box::new(|$arg| $mapper)
                )
            ));
        )*
    };
}

impl <T> ColumnFilter<T> for I32ColumnFilter<T> {
    fn id(&self) -> &str { self.id.as_str() }
    fn get_value(&self, t: &T) -> ScalarValue { ScalarValue::I32((self.mapper)(t)) }
    fn column_filter_state(&self) -> &ColumnFilterState<T> { &self.column_filter_state }
}


pub struct NaiveDateColumnFilter<T> {
    id: String,
    column_filter_state: ColumnFilterState<T>,
    mapper: Box<dyn Fn(&T) -> NaiveDate>
}

impl <T> NaiveDateColumnFilter<T> {
    pub fn new(id: &str, table_filter: Rc<TableFilter<T>>, mapper: Box<dyn Fn(&T) -> NaiveDate>) -> Self {
        Self {
            id: id.to_string(),
            column_filter_state: ColumnFilterState::new(&table_filter),
            mapper,
        }
    }
}

impl <T> ColumnFilter<T> for NaiveDateColumnFilter<T> {
    fn id(&self) -> &str { self.id.as_str() }
    fn get_value(&self, t: &T) -> ScalarValue { ScalarValue::I32((self.mapper)(t).to_epoch_days()) }
    fn column_filter_state(&self) -> &ColumnFilterState<T> { &self.column_filter_state }

    fn get_string_value(&self, t: &T) -> String {
        (self.mapper)(t).format("%-m/%-d/%Y").to_string()
    }
}

pub struct BoolColumnFilter<T> {
    id: String,
    column_filter_state: ColumnFilterState<T>,
    mapper: Box<dyn Fn(&T) -> bool>
}

impl <T> BoolColumnFilter<T> {
    pub fn new(id: &str, table_filter: Rc<TableFilter<T>>, mapper: Box<dyn Fn(&T) -> bool>) -> Self {
        Self {
            id: id.to_string(),
            column_filter_state: ColumnFilterState::new(&table_filter),
            mapper
        }
    }
}

impl <T> ColumnFilter<T> for BoolColumnFilter<T> {
    fn id(&self) -> &str { self.id.as_str() }
    fn get_value(&self, t: &T) -> ScalarValue { ScalarValue::Bool((self.mapper)(t)) }
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
                    Box::new(|$arg| $mapper)
                )
            ));
        )*
    };
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
                    Box::new(|$arg| $mapper)
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