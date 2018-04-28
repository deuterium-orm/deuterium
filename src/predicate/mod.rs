use std::{fmt, rc};

use super::sql;

pub use self::raw::RawPredicate;
pub use self::is::{IsPredicate, ToIsPredicate};
pub use self::is_null::{IsNullPredicate, ToIsNullPredicate};
pub use self::or::{OrPredicate, ToOrPredicate};
pub use self::and::{AndPredicate, ToAndPredicate};
pub use self::exclude::{ExcludePredicate, ToExcludePredicate};
pub use self::like::{LikePredicate, ToLikePredicate};
pub use self::in_::{
    InPredicate, ToInPredicate
};

pub use self::range::{
    InRangePredicate, ToInRangePredicate,
    InRangeBounds
};

pub use self::inequality::{
    InequalityPredicate, ToInequalityPredicate,
    Inequality
};

pub mod or;
pub mod is;
pub mod is_null;
pub mod in_;
pub mod range;
pub mod and;
pub mod inequality;
pub mod exclude;
pub mod like;
pub mod raw;

pub trait Predicate: sql::PredicateToSql + fmt::Debug {}

pub trait ToSharedPredicate {
    fn upcast(self) -> SharedPredicate;
}

impl<T> ToSharedPredicate for T where T: Predicate + Clone + 'static {
    fn upcast(self) -> SharedPredicate {
        rc::Rc::new(Box::new(self))
    }
}

pub type BoxedPredicate = Box<Predicate + 'static>;
pub type SharedPredicate = rc::Rc<BoxedPredicate>;
