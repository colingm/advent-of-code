use aoc2021::{get_input, parse_string};

struct Packet {
    version: u8,
    id: u8,
    is_literal: bool,
    value: usize,
    subpackets: Vec<Packet>
}

fn convert_hex_to_binary(row: &String) -> String {
    row.chars()
        .map(|c|
            match c {
                '0' => "0000",
                '1' => "0001",
                '2' => "0010",
                '3' => "0011",
                '4' => "0100",
                '5' => "0101",
                '6' => "0110",
                '7' => "0111",
                '8' => "1000",
                '9' => "1001",
                'A' => "1010",
                'B' => "1011",
                'C' => "1100",
                'D' => "1101",
                'E' => "1110",
                'F' => "1111",
                _ => "",
            }
        )
        .collect::<Vec<&str>>()
        .join("")
}

fn parse_literal(bits: &String) -> (usize, String) {
    let mut found_end = false;
    let mut value: String = String::new();
    let mut remaining: String = bits.to_string();
    while !found_end {
        let prefix: String = remaining[..1].to_string();
        value += &remaining[1..5];
        remaining = remaining[5..].to_string();
        if prefix == "0" {
            found_end = true;
        }
    }

    (usize::from_str_radix(value.as_str(), 2).unwrap(), remaining)
}

impl Packet {
    fn new (bits: &String) -> (Self, String) {
        let version: u8 = u8::from_str_radix(&bits[..3], 2).unwrap();
        let id: u8 = u8::from_str_radix(&bits[3..6], 2).unwrap();
        let mut rest: String = bits[6..bits.len()].to_string();
        let is_literal: bool;
        let value: usize;
        let mut subpackets: Vec<Packet> = Vec::new();
        if id == 4 {
            is_literal = true;
            let (literal, remaining) = parse_literal(&rest);
            value = literal;
            rest = remaining;
        } else {
            is_literal = false;
            value = 0;
            let length_type_id: u8 = rest[..1].parse().unwrap();
            if length_type_id == 0 {
                let subpacket_length: usize = usize::from_str_radix(&rest[1..16], 2).unwrap();
                let mut sub_bits: String = rest[16..(16+subpacket_length)].to_string();
                rest = rest[(16+subpacket_length)..rest.len()].to_string();
                while sub_bits.len() >= 11 {
                    let (p, r) = Packet::new(&sub_bits);
                    subpackets.push(p);
                    sub_bits = r;
                }
            } else {
                let subpacket_count: usize = usize::from_str_radix(&rest[1..12], 2).unwrap();
                rest = rest[12..rest.len()].to_string();
                for _ in 0..subpacket_count {
                    let (p, r) = Packet::new(&rest);
                    subpackets.push(p);
                    rest = r;
                }
            }
        }

        (Self {
            version,
            id,
            is_literal,
            value,
            subpackets
        }, rest)
    }

    fn version_sum(&self) -> usize {
        if self.is_literal {
            return self.version as usize;
        }

        self.subpackets.iter()
            .map(|p| p.version_sum())
            .sum::<usize>() + self.version as usize
    }

    fn value(&self) -> usize {
        if self.is_literal {
            return self.value;
        }

        match self.id {
            0 => {
                self.subpackets.iter()
                    .map(|p| p.value())
                    .sum()
            },
            1 => {
                self.subpackets.iter()
                    .map(|p| p.value())
                    .product()
            },
            2 => {
                let mut values: Vec<usize> = self.subpackets.iter()
                    .map(|p| p.value())
                    .collect();
                values.sort_unstable();
                values[0]
            },
            3 => {
                let mut values: Vec<usize> = self.subpackets.iter()
                    .map(|p| p.value())
                    .collect();
                values.sort_unstable();
                values.pop().unwrap()
            },
            4 => {
                self.value
            },
            5 => {
                let values: Vec<usize> = self.subpackets.iter()
                    .map(|p| p.value())
                    .collect();
                if values[0] > values[1] { 1 } else { 0 }
            },
            6 => {
                let values: Vec<usize> = self.subpackets.iter()
                    .map(|p| p.value())
                    .collect();
                if values[0] < values[1] { 1 } else { 0 }
            },
            7 => {
                let values: Vec<usize> = self.subpackets.iter()
                    .map(|p| p.value())
                    .collect();
                if values[0] == values[1] { 1 } else { 0 }
            },
            _ => { 0 }
        }
    }
}

fn part_1(rows: &Vec<String>) -> usize {
    let bits = convert_hex_to_binary(&rows[0]);
    let (packet, _): (Packet, String) = Packet::new(&bits);

    packet.version_sum()
}

fn part_2(rows: &Vec<String>) -> usize {
    let bits = convert_hex_to_binary(&rows[0]);
    let (packet, _): (Packet, String) = Packet::new(&bits);

    packet.value()
}

fn main() {
    let input: Vec<String> = get_input("inputs/day_16.txt", &parse_string);

    println!("Part 1 Results: {}", part_1(&input));
    println!("Part 2 Results: {}", part_2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&vec![String::from("D2FE28")]), 6);
        assert_eq!(part_1(&vec![String::from("38006F45291200")]), 9);
        assert_eq!(part_1(&vec![String::from("EE00D40C823060")]), 14);
        assert_eq!(part_1(&vec![String::from("8A004A801A8002F478")]), 16);
        assert_eq!(part_1(&vec![String::from("620080001611562C8802118E34")]), 12);
        assert_eq!(part_1(&vec![String::from("C0015000016115A2E0802F182340")]), 23);
        assert_eq!(part_1(&vec![String::from("A0016C880162017C3686B18A3D4780")]), 31);
    }

    #[test]
    fn test_part_1_answer() {
        let input = get_input("inputs/day_16.txt", &parse_string);

        assert_eq!(part_1(&input), 967);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&vec![String::from("C200B40A82")]), 3);
        assert_eq!(part_2(&vec![String::from("04005AC33890")]), 54);
        assert_eq!(part_2(&vec![String::from("880086C3E88112")]), 7);
        assert_eq!(part_2(&vec![String::from("CE00C43D881120")]), 9);
        assert_eq!(part_2(&vec![String::from("D8005AC2A8F0")]), 1);
        assert_eq!(part_2(&vec![String::from("F600BC2D8F")]), 0);
        assert_eq!(part_2(&vec![String::from("9C005AC2F8F0")]), 0);
        assert_eq!(part_2(&vec![String::from("9C0141080250320F1802104A08")]), 1);
    }

    #[test]
    fn test_part_2_answer() {
        let input = get_input("inputs/day_16.txt", &parse_string);

        assert_eq!(part_2(&input), 12883091136209);
    }

    #[test]
    fn test_parse_literal() {
        let bits = String::from("101111111000101000");

        assert_eq!(parse_literal(&bits), (2021, String::from("000")));
    }
}