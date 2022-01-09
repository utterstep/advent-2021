use crate::{reduce::reduce, Node};

pub fn sum(nodes: &mut [Node]) -> Option<Node> {
    let first = nodes.first()?.clone();

    Some(nodes[1..].iter().fold(first, |res, current| {
        let res = &res + current;
        reduce(&res);

        res
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::node::{node, NodeValue};

    #[test]
    fn test_sum() {
        let mut nodes = [
            node!(1, 1),
            node!(2, 2),
            node!(3, 3),
            node!(4, 4),
        ];

        assert_eq!(
            format!("{}", sum(&mut nodes).unwrap()),
            "[[[[1,1],[2,2]],[3,3]],[4,4]]"
        );

        let mut nodes = [
            node![1,1],
            node![2,2],
            node![3,3],
            node![4,4],
            node![5,5],
            node![6,6],
        ];

        assert_eq!(
            format!("{}", sum(&mut nodes).unwrap()),
            "[[[[5,0],[7,4]],[5,5]],[6,6]]"
        );

        let first: Node = "[[[[4,3],4],4],[7,[[8,4],9]]]".parse().unwrap();
        let second: Node = "[1,1]".parse().unwrap();
        assert_eq!(
            format!("{}", sum(&mut [first, second]).unwrap()),
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        );

        let first: Node = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]".parse().unwrap();
        let second: Node = "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]".parse().unwrap();

        assert_eq!(
            format!("{}", &first),
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]"
        );
        assert_eq!(format!("{}", &second), "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]");

        let res = &first + &second;
        reduce(&res);

        assert_eq!(
            format!("{}", res),
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
        );

        let mut large_example: Vec<Node> = indoc::indoc!(
            "
            [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
            [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
            [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
            [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
            [7,[5,[[3,8],[1,4]]]]
            [[2,[2,2]],[8,[8,1]]]
            [2,9]
            [1,[[[9,3],9],[[9,0],[0,7]]]]
            [[[5,[7,4]],7],1]
            [[[[4,2],2],6],[8,7]]"
        )
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();

        assert_eq!(
            format!("{}", sum(&mut large_example).unwrap()),
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        );
    }
}
