use crate::{node::Node, visitors::{ExplodeVisitor, SplitVisitor}};

pub fn reduce(node: &Node) {
    while ExplodeVisitor::explode(node) || SplitVisitor::split(node) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::node::{node, NodeValue};

    #[test]
    fn test_reduce() {
        // [[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]] -> [[[[0,7],4],[[7,8],[6,0]]],[8,1]]
        let start = node!(
            node!(
                node!(node!(node!(4, 3), 4), 4),
                node!(7, node!(node!(8, 4), 9))
            ),
            node!(1, 1)
        );
        let end = node!(
            node!(node!(node!(0, 7), 4), node!(node!(7, 8), node!(6, 0))),
            node!(8, 1)
        );

        reduce(&start);

        assert_eq!(start.magnitude(), end.magnitude());
    }
}
