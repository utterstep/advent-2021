use crate::node::Node;

pub use explode::ExplodeVisitor;
pub use split::SplitVisitor;

mod explode;
mod split;

trait NodeVisitor<T> {
    fn visit_node(&mut self, node: Node) -> T;
}
