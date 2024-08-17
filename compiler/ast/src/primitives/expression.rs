use crate::primitives::node::Node;
use crate::Visitor;

pub trait Expression : Node {
    fn accept(&self, visitor: &mut dyn Visitor);
    fn clone_boxed(&self) -> Box<dyn Expression>;
}

impl Clone for Box<dyn Expression> {
    fn clone(&self) -> Box<dyn Expression> {
        self.as_ref().clone_boxed()
    }
}