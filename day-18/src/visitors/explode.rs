use super::NodeVisitor;

use crate::node::{Node, NodeSide, NodeValue};

#[derive(Debug)]
pub struct ExplodeVisitor {
    level: usize,
}

impl ExplodeVisitor {
    const EXPLODE_LEVEL: usize = 4;

    fn new() -> Self {
        Self { level: 0 }
    }

    pub fn explode(node: &Node) -> bool {
        let mut visitor = Self::new();

        visitor.visit_node(node.clone())
    }
}

impl NodeVisitor<bool> for ExplodeVisitor {
    fn visit_node(&mut self, mut node: Node) -> bool {
        if self.level == Self::EXPLODE_LEVEL {
            node.explode();

            return true;
        }
        self.level += 1;

        let exploded = match (node.child(NodeSide::Left), node.child(NodeSide::Right)) {
            (NodeValue::Node(left), NodeValue::Node(right)) => {
                self.visit_node(left) || self.visit_node(right)
            }
            (NodeValue::Node(node), _) | (_, NodeValue::Node(node)) => self.visit_node(node),
            _ => false,
        };

        if !exploded {
            self.level -= 1;
        }

        exploded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::node::node;

    #[test]
    fn test_explode_visitor() {
        // [[6,[5,[4,[3,2]]]],1] -> [[6,[5,[7,0]]],3]
        let l0 = node!(node!(6, node!(5, node!(4, node!(3, 2)))), 1);

        assert!(ExplodeVisitor::explode(&l0));
        assert_eq!(l0.magnitude(), 402);

        assert!(!ExplodeVisitor::explode(&l0));

        // [[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]] -> [[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]
        let l0 = node!(
            node!(3, node!(2, node!(1, node!(7, 3)))),
            node!(6, node!(5, node!(4, node!(3, 2))))
        );
        let to_be = node!(
            node!(3, node!(2, node!(8, 0))),
            node!(9, node!(5, node!(4, node!(3, 2))))
        );

        assert!(ExplodeVisitor::explode(&l0));
        assert_eq!(l0.magnitude(), to_be.magnitude());

        let node: Node = "[1,[2,[[0,[11,3]],[[6,3],[8,8]]]]]".parse().unwrap();

        assert!(ExplodeVisitor::explode(&node));
        assert_eq!(format!("{}", &node), "[1,[2,[[11,0],[[9,3],[8,8]]]]]",);
    }
}
