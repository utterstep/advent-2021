use super::{Operator, Packet, PacketPayload};

pub trait Visitor<T> {
    fn visit_packet(&mut self, packet: &Packet) -> T;

    fn visit_payload(&mut self, payload: &PacketPayload) -> T;
}

#[derive(Debug, Default)]
pub struct VersionSumVisitor {}

impl Visitor<u64> for VersionSumVisitor {
    fn visit_packet(&mut self, packet: &Packet) -> u64 {
        packet.version as u64 + self.visit_payload(&packet.payload)
    }

    fn visit_payload(&mut self, payload: &PacketPayload) -> u64 {
        match payload {
            PacketPayload::Literal(_) => 0,
            PacketPayload::Expression {
                operator: _operator,
                operands: packets,
            } => packets.iter().map(|packet| self.visit_packet(packet)).sum(),
        }
    }
}

#[derive(Debug, Default)]
pub struct ExpressionEvalVisitor {}

impl Visitor<u64> for ExpressionEvalVisitor {
    fn visit_packet(&mut self, packet: &Packet) -> u64 {
        self.visit_payload(&packet.payload)
    }

    fn visit_payload(&mut self, payload: &PacketPayload) -> u64 {
        match payload {
            PacketPayload::Literal(value) => *value,
            PacketPayload::Expression { operator, operands } => {
                macro_rules! get_two {
                    ($operands: expr) => {
                        // safe to unwrap – checked operands count during construction
                        ($operands.next().unwrap(), $operands.next().unwrap())
                    };
                }

                let mut operands = operands.iter().map(|packet| self.visit_packet(packet));

                match *operator {
                    Operator::Sum => operands.sum(),
                    Operator::Product => operands.product(),
                    Operator::Minimum => operands.min().unwrap_or(0),
                    Operator::Maximum => operands.max().unwrap_or(0),
                    Operator::GreaterThan => {
                        let (first, second) = get_two!(operands);

                        (first > second) as u64
                    }
                    Operator::LessThan => {
                        let (first, second) = get_two!(operands);

                        (first < second) as u64
                    }
                    Operator::EqualTo => {
                        let (first, second) = get_two!(operands);

                        (first == second) as u64
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_sum_examples() {
        macro_rules! assert_version_sum_eq {
            ($packet:literal, $expected:literal) => {
                let packet = $packet.parse().unwrap();
                let mut visitor = VersionSumVisitor::default();

                assert_eq!(visitor.visit_packet(&packet), $expected);
            };
        }

        assert_version_sum_eq!("D2FE28", 6);
        assert_version_sum_eq!("38006F45291200", 9);
        assert_version_sum_eq!("EE00D40C823060", 14);
        assert_version_sum_eq!("8A004A801A8002F478", 16);
        assert_version_sum_eq!("620080001611562C8802118E34", 12);
        assert_version_sum_eq!("C0015000016115A2E0802F182340", 23);
        assert_version_sum_eq!("A0016C880162017C3686B18A3D4780", 31);
    }

    #[test]
    fn test_expr_eval_examples() {
        macro_rules! assert_eval_expr_eq {
            ($packet:literal, $expected:literal) => {
                let packet = $packet.parse().unwrap();
                let mut visitor = ExpressionEvalVisitor::default();

                assert_eq!(visitor.visit_packet(&packet), $expected);
            };
        }

        assert_eval_expr_eq!("C200B40A82", 3);
        assert_eval_expr_eq!("04005AC33890", 54);
        assert_eval_expr_eq!("880086C3E88112", 7);
        assert_eval_expr_eq!("CE00C43D881120", 9);
        assert_eval_expr_eq!("D8005AC2A8F0", 1);
        assert_eval_expr_eq!("F600BC2D8F", 0);
        assert_eval_expr_eq!("9C005AC2F8F0", 0);
        assert_eval_expr_eq!("9C0141080250320F1802104A08", 1);
    }
}
