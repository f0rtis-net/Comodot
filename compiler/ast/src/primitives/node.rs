use crate::Visitor;

pub trait Node {
    fn get_literal(&self) -> String;
}