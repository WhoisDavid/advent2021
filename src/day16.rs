use aoc_runner_derive::{aoc, aoc_generator};
use bitvec::prelude::*;
use itertools::Itertools;

#[aoc_generator(day16)]
pub fn input_parser(input: &str) -> BitVec<Msb0> {
    let bit_count = 4 * input.chars().count();
    let mut bv: BitVec<Msb0> = input
        .chars()
        .chain(std::iter::repeat('0').take(16 - bit_count % 16)) // pad to 64 bits
        .chunks(16)
        .into_iter()
        .map(|s| s.fold(0, |v, hex| (v << 4) + hex.to_digit(16).unwrap() as usize))
        .collect();
    bv.truncate(bit_count);
    bv
}

#[derive(Debug)]
pub enum Payload {
    Literal(u64),
    Operator(Operator),
}

#[derive(Debug)]
pub enum SubpacketsSize {
    Length(u16),
    Number(u16),
}

#[derive(Debug)]
pub struct Operator {
    _length_type_id: u16,
    _subpackets_size: SubpacketsSize,
    subpackets: Vec<Packet>,
}

#[derive(Debug)]
pub struct Packet {
    version: u16,
    type_id: u16,
    payload: Payload,
    packet_size: usize,
}

fn parse_packet(input: &BitSlice<Msb0>) -> Packet {
    let mut idx = 0;

    let mut next_bits = |n: usize| {
        let res = input[idx..idx + n].load_be();
        idx += n;
        res
    };

    let version = next_bits(3);
    let type_id = next_bits(3);
    let payload = if type_id == 4 {
        // literal value
        let mut literal_val = 0;
        // leading 1
        while next_bits(1) == 1 {
            literal_val = (literal_val << 4) + next_bits(4) as u64;
        }
        // leading 0
        literal_val = (literal_val << 4) + next_bits(4) as u64;

        Payload::Literal(literal_val)
    } else {
        // operator
        let length_type_id = next_bits(1);
        if length_type_id == 0 {
            let bit_length = next_bits(15);
            let subpackets_size = SubpacketsSize::Length(bit_length);
            let end = idx + bit_length as usize;

            let mut subpackets = Vec::new();
            while idx < end {
                let subpacket = parse_packet(&input[idx..end]);
                idx += subpacket.packet_size;
                subpackets.push(subpacket);
            }

            let op = Operator {
                _length_type_id: length_type_id,
                _subpackets_size: subpackets_size,
                subpackets,
            };

            Payload::Operator(op)
        } else {
            let number_packets = next_bits(11);
            let subpackets_size = SubpacketsSize::Number(number_packets);
            let mut subpackets = Vec::with_capacity(number_packets as usize);

            for _subpacket in 0..number_packets {
                let subpacket = parse_packet(&input[idx..]);
                idx += subpacket.packet_size;
                subpackets.push(subpacket);
            }

            let op = Operator {
                _length_type_id: length_type_id,
                _subpackets_size: subpackets_size,
                subpackets,
            };

            Payload::Operator(op)
        }
    };

    Packet {
        version,
        type_id,
        payload,
        packet_size: idx,
    }
}

fn sum_versions(p: &Packet) -> u64 {
    let mut res = p.version as u64;
    if let Payload::Operator(op) = &p.payload {
        op.subpackets.iter().for_each(|p| res += sum_versions(p));
    }
    res
}

fn calculate_expr(packet: &Packet) -> u64 {
    match &packet.payload {
        Payload::Literal(lit) => *lit as u64,
        Payload::Operator(op) => {
            let subexpr = op.subpackets.iter().map(|p| calculate_expr(p));
            match packet.type_id {
                0 => subexpr.sum(),
                1 => subexpr.product(),
                2 => subexpr.min().unwrap(),
                3 => subexpr.max().unwrap(),
                cmp @ (5 | 6 | 7) => {
                    assert!(op.subpackets.len() == 2);
                    let p1 = calculate_expr(&op.subpackets[0]);
                    let p2 = calculate_expr(&op.subpackets[1]);
                    let b = match cmp {
                        5 => p1 > p2,
                        6 => p1 < p2,
                        7 => p1 == p2,
                        _ => unreachable!(),
                    };
                    b as u64
                }
                _ => unreachable!(),
            }
        }
    }
}

#[aoc(day16, part1)]
pub fn part1(input: &BitVec<Msb0>) -> u64 {
    let packet = parse_packet(input);
    sum_versions(&packet)
}

#[aoc(day16, part2)]
pub fn part2(input: &BitVec<Msb0>) -> u64 {
    let packet = parse_packet(input);
    calculate_expr(&packet)
}

#[cfg(test)]
mod test_day16 {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser("8A004A801A8002F478")), 16);
        assert_eq!(part1(&input_parser("620080001611562C8802118E34")), 12);
        assert_eq!(part1(&input_parser("C0015000016115A2E0802F182340")), 23);
        assert_eq!(part1(&input_parser("A0016C880162017C3686B18A3D4780")), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_parser("C200B40A82")), 3);
        assert_eq!(part2(&input_parser("04005AC33890")), 54);
        assert_eq!(part2(&input_parser("880086C3E88112")), 7);
        assert_eq!(part2(&input_parser("CE00C43D881120")), 9);
        assert_eq!(part2(&input_parser("D8005AC2A8F0")), 1);
        assert_eq!(part2(&input_parser("F600BC2D8F")), 0);
        assert_eq!(part2(&input_parser("9C005AC2F8F0")), 0);
        assert_eq!(part2(&input_parser("9C0141080250320F1802104A08")), 1);
    }
}
