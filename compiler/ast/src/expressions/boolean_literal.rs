use crate::primitives::expression::Expression;
use crate::primitives::node::Node;
use crate::Visitor;

#[derive(Clone)]
pub struct BooleanLiteral {
    pub val: bool
}

impl Node for BooleanLiteral {
    fn get_literal(&self) -> String {
        self.val.to_string()
    }
}

impl Expression for BooleanLiteral {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_boolean_literal(self)
    }

    fn clone_boxed(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}