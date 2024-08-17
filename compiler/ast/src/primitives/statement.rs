use crate::primitives::node::Node;
use crate::Visitor;

pub trait Statement : Node {
    fn accept(&self, visitor: &mut dyn Visitor);
    fn clone_boxed(&self) -> Box<dyn Statement>;
}

impl Clone for Box<dyn Statement> {
    fn clone(&self) -> Box<dyn Statement> {
        self.as_ref().clone_boxed()
    }
}