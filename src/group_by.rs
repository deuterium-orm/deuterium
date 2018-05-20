use super::expression;

#[derive(Clone, Debug)]
pub struct GroupBy {
    by: Vec<expression::SharedExpression>
}

impl GroupBy {
    pub fn get_by(&self) -> &Vec<expression::SharedExpression> { &self.by }
    pub fn by(fields: &[&expression::UntypedExpression]) -> GroupBy {
        GroupBy { by: fields.iter().map(|f| f.upcast_expression()).collect() }
    }
}
