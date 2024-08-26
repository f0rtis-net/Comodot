use crate::Visitor;

pub trait Node {
    fn accept(&self, visitor: &mut dyn Visitor);
    fn clone_boxed(&self) -> Box<dyn Node>;
    fn get_literal(&self) -> String;
}

impl Clone for Box<dyn Node> {
    fn clone(&self) -> Box<dyn Node> {
        self.as_ref().clone_boxed()
    }
}