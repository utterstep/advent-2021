use std::{
    cell::RefCell,
    fmt::Display,
    ops::Add,
    rc::{Rc, Weak},
};

mod parser;

#[derive(Debug, Clone)]
pub struct NodeContent {
    parent: Option<ParentNode>,
    left: NodeValue,
    right: NodeValue,
}

type Link = Rc<RefCell<NodeContent>>;
type WeakLink = Weak<RefCell<NodeContent>>;

#[derive(Debug, Clone)]
pub struct Node(Link);

#[derive(Debug, Clone)]
struct WeakNode(WeakLink);

impl Add for &Node {
    type Output = Node;

    fn add(self, rhs: Self) -> Self::Output {
        self.add(rhs)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_borrow = self.0.borrow();

        let left = match &self_borrow.left {
            NodeValue::Literal(value) => format!("{}", value),
            NodeValue::Node(node) => format!("{}", node),
        };
        let right = match &self_borrow.right {
            NodeValue::Literal(value) => format!("{}", value),
            NodeValue::Node(node) => format!("{}", node),
        };

        write!(f, "[{},{}]", left, right)
    }
}

impl Node {
    fn downgrade(&self) -> WeakNode {
        WeakNode(Rc::downgrade(&self.0))
    }

    pub fn new(left: NodeValue, right: NodeValue) -> Self {
        let this = Node(Rc::new(RefCell::new(NodeContent {
            parent: None,
            left,
            right,
        })));

        (*this.0.borrow_mut())
            .left
            .set_parent(&this, NodeSide::Left);
        (*this.0.borrow_mut())
            .right
            .set_parent(&this, NodeSide::Right);

        this
    }

    fn add(&self, rhs: &Self) -> Self {
        Self::new(NodeValue::Node(self.clone()), NodeValue::Node(rhs.clone()))
    }

    pub fn magnitude(&self) -> u64 {
        let left_magnitude = match &self.0.borrow().left {
            NodeValue::Literal(value) => *value,
            NodeValue::Node(node) => node.magnitude(),
        };

        let right_magnitude = match &self.0.borrow().right {
            NodeValue::Literal(value) => *value,
            NodeValue::Node(node) => node.magnitude(),
        };

        3 * left_magnitude + 2 * right_magnitude
    }

    fn add_literal_left_up(&self, inc: u64, from: NodeSide) {
        match from {
            NodeSide::Left => {
                if let Some(parent_node) = &self.0.borrow().parent {
                    parent_node
                        .node()
                        .expect("invalid weak ref")
                        .add_literal_left_up(inc, parent_node.side);
                }
            }
            NodeSide::Right => match &mut self.0.borrow_mut().left {
                literal @ &mut NodeValue::Literal(_) => {
                    *literal = NodeValue::Literal(literal.value().unwrap() + inc)
                }
                NodeValue::Node(node) => {
                    node.add_literal_left_down(inc);
                }
            },
        }
    }

    fn add_literal_left_down(&self, inc: u64) {
        match &mut self.0.borrow_mut().right {
            literal @ &mut NodeValue::Literal(_) => {
                *literal = NodeValue::Literal(literal.value().unwrap() + inc)
            }
            NodeValue::Node(node) => {
                node.add_literal_left_down(inc);
            }
        }
    }

    fn add_literal_right_up(&self, inc: u64, from: NodeSide) {
        match from {
            NodeSide::Right => {
                if let Some(parent_node) = &self.0.borrow().parent {
                    parent_node
                        .node()
                        .expect("invalid weak ref")
                        .add_literal_right_up(inc, parent_node.side);
                }
            }
            NodeSide::Left => match &mut self.0.borrow_mut().right {
                literal @ &mut NodeValue::Literal(_) => {
                    *literal = NodeValue::Literal(literal.value().unwrap() + inc)
                }
                NodeValue::Node(node) => {
                    node.add_literal_right_down(inc);
                }
            },
        }
    }

    fn add_literal_right_down(&self, inc: u64) {
        match &mut self.0.borrow_mut().left {
            literal @ &mut NodeValue::Literal(_) => {
                *literal = NodeValue::Literal(literal.value().unwrap() + inc)
            }
            NodeValue::Node(node) => {
                node.add_literal_right_down(inc);
            }
        }
    }

    fn replace_child(&mut self, mut child: NodeValue, side: NodeSide) {
        child.set_parent(self, side);

        let mut self_borrow_mut = self.0.borrow_mut();

        match side {
            NodeSide::Left => {
                (*self_borrow_mut).left = child;
            }
            NodeSide::Right => {
                (*self_borrow_mut).right = child;
            }
        }
    }

    pub(crate) fn child(&self, side: NodeSide) -> NodeValue {
        let self_borrow = self.0.borrow();

        match side {
            NodeSide::Left => self_borrow.left.clone(),
            NodeSide::Right => self_borrow.right.clone(),
        }
    }

    pub(crate) fn explode(&mut self) -> Option<Node> {
        let left_value = self.0.borrow().left.value()?;

        let right_value = self.0.borrow().right.value()?;

        let self_borrow = &self.0.borrow_mut();
        let parent_node = self_borrow.parent.as_ref()?;

        let mut node = parent_node.node()?;

        node.add_literal_left_up(left_value, parent_node.side);
        node.add_literal_right_up(right_value, parent_node.side);

        node.replace_child(NodeValue::Literal(0), parent_node.side);

        Some(node)
    }

    pub(crate) fn literal(&self, side: NodeSide) -> Option<u64> {
        match side {
            NodeSide::Left => self.0.borrow().left.value(),
            NodeSide::Right => self.0.borrow().right.value(),
        }
    }

    pub(crate) fn split(&mut self, side: NodeSide) {
        let value = self.literal(side).expect("splitting non-leaf node");

        self.replace_child(
            NodeValue::Node(Node::new(
                NodeValue::Literal(value / 2),
                NodeValue::Literal(value / 2 + (value & 1)),
            )),
            side,
        );
    }
}

#[cfg(test)]
macro_rules! node {
    ($left: literal, $right: literal) => {{
        let node = Node::new(NodeValue::Literal($left), NodeValue::Literal($right));

        node
    }};
    ($left: expr, $right: literal) => {{
        let new = Node::new(NodeValue::Node($left), NodeValue::Literal($right));

        new
    }};
    ($left: literal, $right: expr) => {{
        let new = Node::new(NodeValue::Literal($left), NodeValue::Node($right));

        new
    }};
    ($left: expr, $right: expr) => {{
        let new = Node::new(NodeValue::Node($left), NodeValue::Node($right));

        new
    }};
}

#[cfg(test)]
pub(crate) use node;

#[derive(Debug, Clone)]
pub struct ParentNode {
    node: WeakNode,
    side: NodeSide,
}

impl ParentNode {
    fn node(&self) -> Option<Node> {
        Some(Node(self.node.0.upgrade()?))
    }
}

#[derive(Debug, Clone)]
pub enum NodeValue {
    Literal(u64),
    Node(Node),
}

impl NodeValue {
    fn set_parent(&mut self, parent: &Node, side: NodeSide) {
        if let Self::Node(node) = self {
            let new_parent = ParentNode {
                node: parent.downgrade(),
                side,
            };

            let mut new_node_content = node.0.borrow().clone();
            new_node_content.parent = Some(new_parent);

            *node.0.borrow_mut() = new_node_content;
        }
    }

    fn value(&self) -> Option<u64> {
        if let Self::Literal(value) = self {
            Some(*value)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum NodeSide {
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_impl() {
        let node_a = Node(Rc::new(RefCell::new(NodeContent {
            parent: None,
            left: NodeValue::Literal(6),
            right: NodeValue::Literal(5),
        })));

        let node_b = Node(Rc::new(RefCell::new(NodeContent {
            parent: None,
            left: NodeValue::Literal(7),
            right: NodeValue::Literal(8),
        })));

        let sum = &node_a + &node_b;

        assert!(sum.0.borrow().parent.is_none());
        assert!(matches!(sum.0.borrow().left, NodeValue::Node(_)));
        assert!(matches!(sum.0.borrow().right, NodeValue::Node(_)));

        assert_eq!(node_a.magnitude(), 6 * 3 + 5 * 2);
        assert_eq!(node_b.magnitude(), 7 * 3 + 8 * 2);
        assert_eq!(
            sum.magnitude(),
            node_a.magnitude() * 3 + node_b.magnitude() * 2
        );
    }

    #[test]
    fn test_explode_impl() {
        let mut nodes = vec![];

        macro_rules! node {
            ($left: literal, $right: literal) => {{
                let new = Node::new(NodeValue::Literal($left), NodeValue::Literal($right));

                nodes.push(new.clone());

                new
            }};
            ($left: expr, $right: literal) => {{
                let new = Node::new(NodeValue::Node($left), NodeValue::Literal($right));

                nodes.push(new.clone());

                new
            }};
            ($left: literal, $right: expr) => {{
                let new = Node::new(NodeValue::Literal($left), NodeValue::Node($right));

                nodes.push(new.clone());

                new
            }};
            ($left: expr, $right: expr) => {{
                let new = Node::new(NodeValue::Node($left), NodeValue::Node($right));

                nodes.push(new.clone());

                new
            }};
        }

        // [[[[[9,8],1],2],3],4] -> [[[[0,9],2],3],4]

        let l0 = node!(node!(node!(node!(node!(9, 8), 1), 2), 3), 4);
        assert_eq!(l0.magnitude(), 3599);

        nodes[0].explode();
        assert_eq!(l0.magnitude(), 548);

        nodes.clear();

        // [7,[6,[5,[4,[3,2]]]]] -> [7,[6,[5,[7,0]]]]

        let l0 = node!(7, node!(6, node!(5, node!(4, node!(3, 2)))));
        assert_eq!(l0.magnitude(), 421);

        nodes[0].explode();
        assert_eq!(l0.magnitude(), 285);

        nodes.clear();

        // [[6,[5,[4,[3,2]]]],1] -> [[6,[5,[7,0]]],3]

        let l0 = node!(node!(6, node!(5, node!(4, node!(3, 2)))), 1);
        assert_eq!(l0.magnitude(), 602);

        nodes[0].explode();
        assert_eq!(l0.magnitude(), 402);
    }

    #[test]
    fn test_split_impl() {
        let mut nodes = vec![];

        macro_rules! node {
            ($left: literal, $right: literal) => {{
                let node = Node::new(NodeValue::Literal($left), NodeValue::Literal($right));

                nodes.push(node.clone());

                node
            }};
            ($left: expr, $right: literal) => {{
                let new = Node::new(NodeValue::Node($left), NodeValue::Literal($right));

                nodes.push(new.clone());

                new
            }};
            ($left: literal, $right: expr) => {{
                let new = Node::new(NodeValue::Literal($left), NodeValue::Node($right));

                nodes.push(new.clone());

                new
            }};
            ($left: expr, $right: expr) => {{
                let new = Node::new(NodeValue::Node($left), NodeValue::Node($right));

                nodes.push(new.clone());

                new
            }};
        }

        // [[[[0,7],4],[15,[0,13]]],[1,1]] -> [[[[0,7],4],[[7,8],[0,13]]],[1,1]]

        let l0 = node!(
            node!(node!(node!(0, 7), 4), node!(15, node!(0, 13))),
            node!(1, 1)
        );

        nodes[3].split(NodeSide::Left);
        nodes.clear();

        let expected = node!(
            node!(node!(node!(0, 7), 4), node!(node!(7, 8), node!(0, 13))),
            node!(1, 1)
        );

        assert_eq!(l0.magnitude(), expected.magnitude());
    }
}
