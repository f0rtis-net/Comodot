use crate::primitives::node::Node;
use crate::Visitor;

#[derive(Clone)]
pub struct BooleanLiteral {
    pub val: bool
}

impl Node for BooleanLiteral {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_boolean_literal(self)
    }

    fn clone_boxed(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn get_literal(&self) -> String {
        self.val.to_string()
    }
}
