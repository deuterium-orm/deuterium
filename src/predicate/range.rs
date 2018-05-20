use chrono;
use super::super::sql;
use super::super::expression;
use super::super::field;

use super::ToSharedPredicate;

#[derive(Clone, Copy, Debug)]
pub enum InRangeBounds {
    ExcludeBoth,
    IncludeBoth,
    ExcludeRight,
    ExcludeLeft
}

#[derive(Clone, Debug)]
pub struct InRangePredicate<F, T1, T2> {
    field: F,
    from: T1,
    to: T2,
    bounds: InRangeBounds
}

impl<F, T1, T2> InRangePredicate<F, T1, T2> {
    pub fn get_field(&self) -> &F { &self.field }
    pub fn get_from(&self) -> &T1 { &self.from }
    pub fn get_to(&self) -> &T2 { &self.to }
    pub fn get_bounds(&self) -> &InRangeBounds { &self.bounds }
}

pub trait ToInRangePredicate<T> {
    fn in_range<B1, B2>(&self, from: B1, to: B2) -> super::SharedPredicate
        where B1: expression::ToExpression<T> + sql::ToPredicateValue + Clone + 'static,
              B2: expression::ToExpression<T> + sql::ToPredicateValue + Clone + 'static;

    fn in_range_exclude_left<B1, B2>(&self, from: B1, to: B2) -> super::SharedPredicate
        where B1: expression::ToExpression<T> + sql::ToPredicateValue + Clone + 'static,
              B2: expression::ToExpression<T> + sql::ToPredicateValue + Clone + 'static;

    fn in_range_exclude_right<B1, B2>(&self, from: B1, to: B2) -> super::SharedPredicate
        where B1: expression::ToExpression<T> + sql::ToPredicateValue + Clone + 'static,
              B2: expression::ToExpression<T> + sql::ToPredicateValue + Clone + 'static;

    fn in_range_exclude<B1, B2>(&self, from: B1, to: B2) -> super::SharedPredicate
        where B1: expression::ToExpression<T> + sql::ToPredicateValue + Clone + 'static,
              B2: expression::ToExpression<T> + sql::ToPredicateValue + Clone + 'static;
}

impl<F, T1, T2> super::Predicate for InRangePredicate<F, T1, T2>
    where F: sql::ToPredicateValue,
          T1: sql::ToPredicateValue,
          T2: sql::ToPredicateValue
    { }

macro_rules! impl_for {
    ($field:ty, $expr:ty) => (

        impl ToInRangePredicate<$expr> for $field {
            fn in_range<B1, B2>(&self, from: B1, to: B2) -> super::SharedPredicate
                where B1: expression::ToExpression<$expr> + sql::ToPredicateValue + Clone + 'static,
                      B2: expression::ToExpression<$expr> + sql::ToPredicateValue + Clone + 'static {
                InRangePredicate { field: self.clone(), from: from, to: to, bounds: InRangeBounds::IncludeBoth }.upcast()
            }

            fn in_range_exclude_left<B1, B2>(&self, from: B1, to: B2) -> super::SharedPredicate
                where B1: expression::ToExpression<$expr> + sql::ToPredicateValue + Clone + 'static,
                      B2: expression::ToExpression<$expr> + sql::ToPredicateValue + Clone + 'static {
                InRangePredicate { field: self.clone(), from: from, to: to, bounds: InRangeBounds::ExcludeLeft }.upcast()
            }

            fn in_range_exclude_right<B1, B2>(&self, from: B1, to: B2) -> super::SharedPredicate
                where B1: expression::ToExpression<$expr> + sql::ToPredicateValue + Clone + 'static,
                      B2: expression::ToExpression<$expr> + sql::ToPredicateValue + Clone + 'static {
                InRangePredicate { field: self.clone(), from: from, to: to, bounds: InRangeBounds::ExcludeRight }.upcast()
            }

            fn in_range_exclude<B1, B2>(&self, from: B1, to: B2) -> super::SharedPredicate
                where B1: expression::ToExpression<$expr> + sql::ToPredicateValue + Clone + 'static,
                      B2: expression::ToExpression<$expr> + sql::ToPredicateValue + Clone + 'static {
                InRangePredicate { field: self.clone(), from: from, to: to, bounds: InRangeBounds::ExcludeBoth }.upcast()
            }
        }

    )
}


impl_for!(field::I8Field, i8);
impl_for!(field::I16Field, i16);
impl_for!(field::I32Field, i32);
impl_for!(field::I64Field, i64);
impl_for!(field::F32Field, f32);
impl_for!(field::F64Field, f64);
impl_for!(field::TimespecField, chrono::NaiveDateTime);

impl_for!(field::OptionalI8Field, Option<i8>);
impl_for!(field::OptionalI16Field, Option<i16>);
impl_for!(field::OptionalI32Field, Option<i32>);
impl_for!(field::OptionalI64Field, Option<i64>);
impl_for!(field::OptionalF32Field, Option<f32>);
impl_for!(field::OptionalF64Field, Option<f64>);
impl_for!(field::OptionalTimespecField, Option<chrono::NaiveDateTime>);

impl_for!(expression::RawExpression, expression::RawExpression);
