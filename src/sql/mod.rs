pub use self::predicate::PredicateToSql;
pub use self::value::ToPredicateValue;
#[cfg(feature = "postgres")] pub use self::value::AsPostgresValue;
pub use self::from::FromToSql;
pub use self::adapter::{
    SqlAdapter,
    PostgreSqlAdapter,
    MysqlAdapter,
};

pub mod delete;
pub mod distinct;
pub mod expr_value;
pub mod field;
pub mod function;
pub mod group_by;
pub mod insert;
pub mod join;
pub mod order_by;
pub mod placeholder;
pub mod predicate;
pub mod select;
pub mod from;
pub mod update;
pub mod value;
pub mod adapter;

pub trait QueryToSql: ToSql {
    fn to_final_sql(&self, ctx: &mut SqlContext) -> String {
        let mut sql = format!("{};", self.to_sql(ctx));
        let mut idx = ctx.get_expl_placeholders_count() + 1;
        for i in 0..ctx.get_impl_placeholders_count() {
            sql = sql.replace(&format!("$${}", i), &ctx.adapter().placeholder(idx));
            idx += 1
        }

        sql
    }
}

pub trait ToSql {
    fn to_sql(&self, ctx: &mut SqlContext) -> String;
}

#[cfg(feature = "postgres")]
pub type BoxedValue = Box<::postgres::types::ToSql + 'static>;
#[cfg(not(feature = "postgres"))]
pub type BoxedValue = Box<ToPredicateValue>;
pub type BoxedAdapter = Box<SqlAdapter + 'static>;

#[derive(Debug)]
pub struct SqlContext {
    impl_placeholders: u8,
    expl_placeholders: u8,
    placeholder_data: Vec<BoxedValue>,
    adapter: Box<SqlAdapter + 'static>,
}

impl SqlContext {
    pub fn new(adapter: Box<SqlAdapter + 'static>) -> SqlContext {
        SqlContext {
            impl_placeholders: 0u8,
            expl_placeholders: 0u8,
            placeholder_data: vec![],
            adapter: adapter,
        }
    }

    pub fn hold(&mut self, val: BoxedValue) -> String {
        self.placeholder_data.push(val);
        let res = format!("$${}", self.impl_placeholders);
        self.impl_placeholders += 1;
        res
    }

    pub fn get_impl_placeholders_count(&self) -> u8 {
        self.impl_placeholders
    }

    pub fn get_expl_placeholders_count(&self) -> u8 {
        self.expl_placeholders
    }

    pub fn adapter(&self) -> &BoxedAdapter {
        &self.adapter
    }

    pub fn expl_indexed_placeholder(&mut self, idx: u8) {
        if idx > self.expl_placeholders { self.expl_placeholders = idx; }
    }

    pub fn data(&self) -> &[BoxedValue] {
        &self.placeholder_data
    }
}
