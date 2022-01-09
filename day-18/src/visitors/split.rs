use super::NodeVisitor;

use crate::node::{Node, NodeSide, NodeValue};

#[derive(Debug)]
pub struct SplitVisitor {}

impl SplitVisitor {
    const VALUE_LIMIT: u64 = 9;

    fn new() -> Self {
        Self {}
    }

    pub fn split(node: &Node) -> bool {
        let mut visitor = Self::new();

        visitor.visit_node(node.clone())
    }
}

impl NodeVisitor<bool> for SplitVisitor {
    fn visit_node(&mut self, mut node: Node) -> bool {
        for side in [NodeSide::Left, NodeSide::Right] {
            match node.child(side) {
                NodeValue::Literal(value) => {
                    if value > Self::VALUE_LIMIT {
                        node.split(side);

                        return true;
                    }
                }
                NodeValue::Node(node) => {
                    if self.visit_node(node) {
                        return true;
                    }
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::node::node;

    #[test]
    fn test_split_visitor() {
        // [[[[0,7],4],[15,[0,13]]],[1,1]] -> [[[[0,7],4],[[7,8],[0,13]]],[1,1]]

        let l0 = node!(
            node!(node!(node!(0, 7), 4), node!(15, node!(0, 13))),
            node!(1, 1)
        );
        let expected = node!(
            node!(node!(node!(0, 7), 4), node!(node!(7, 8), node!(0, 13))),
            node!(1, 1)
        );

        assert!(SplitVisitor::split(&l0));
        assert_eq!(l0.magnitude(), expected.magnitude());

        // [[[[0,7],4],[[7,8],[0,13]]],[1,1]] -> [[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]
        let expected = node!(
            node!(
                node!(node!(0, 7), 4),
                node!(node!(7, 8), node!(0, node!(6, 7)))
            ),
            node!(1, 1)
        );

        assert!(SplitVisitor::split(&l0));
        assert_eq!(l0.magnitude(), expected.magnitude());

        assert!(!SplitVisitor::split(&l0));
    }
}
