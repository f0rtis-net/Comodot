use crate::primitives::expression::Expression;
use crate::primitives::node::Node;
use crate::Visitor;

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub value: i64
}

impl Node for IntegerLiteral {
    fn get_literal(&self) -> String {
        String::from(self.value.to_string())
    }
}

impl Expression for IntegerLiteral {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_integer_literal(self)
    }

    fn clone_boxed(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}