use std::num::ParseIntError;

static INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Packet {
    version: u64,
    type_id: u64,
    payload: Payload,
}
#[derive(Debug)]
enum Payload {
    Literal(u64),
    Operator(Vec<Packet>),
}

impl Packet {
    fn version_sum(&self) -> u64 {
        self.version
            + match &self.payload {
                Payload::Literal(_) => 0,
                Payload::Operator(packets) => {
                    packets.iter().map(|packet| packet.version_sum()).sum()
                }
            }
    }

    fn apply(&self) -> u64 {
        match &self.payload {
            Payload::Literal(value) => *value,
            Payload::Operator(packets) => match self.type_id {
                0 => packets.iter().map(|p| p.apply()).sum(),
                1 => packets.iter().map(|p| p.apply()).product(),
                2 => packets.iter().map(|p| p.apply()).min().unwrap(),
                3 => packets.iter().map(|p| p.apply()).max().unwrap(),
                5 => {
                    if packets.get(0).unwrap().apply() > packets.get(1).unwrap().apply() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if packets.get(0).unwrap().apply() < packets.get(1).unwrap().apply() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if packets.get(0).unwrap().apply() == packets.get(1).unwrap().apply() {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("unknown type id: {}", self.type_id),
            },
        }
    }
}

struct HexReader {
    packet: String,
    position: usize,
}

impl HexReader {
    fn new(hex_string: String) -> HexReader {
        HexReader {
            packet: hex_to_binary(hex_string.trim()).unwrap(),
            position: 0,
        }
    }

    fn read_binary(&mut self, length: usize) -> String {
        if !self.packet.is_char_boundary(self.position) {
            panic!("unable to read character at position: {}", self.position);
        }

        if !self.packet.is_char_boundary(self.position + length) {
            panic!(
                "unable to read character at position+length: {}+{}",
                self.position, length
            );
        }

        let slice = &self.packet[self.position..self.position + length];
        self.position += length;
        slice.to_string()
    }

    fn read_decimal(&mut self, length: usize) -> u64 {
        let slice = self.read_binary(length);
        binary_to_decimal(&slice).unwrap()
    }
}

fn binary_to_decimal(value: &str) -> Result<u64, ParseIntError> {
    u64::from_str_radix(value, 2)
}

fn hex_to_binary(value: &str) -> Result<String, ParseIntError> {
    Ok(value
        .chars()
        .map(|character| match character {
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
            _ => panic!("unknown single character hexidecmal value: '{}'", character),
        })
        .collect())
}

fn parse_packet(mut reader: HexReader) -> (Packet, HexReader) {
    let version = reader.read_decimal(3);
    let type_id = reader.read_decimal(3);
    match type_id {
        4 => {
            let mut binary = "".to_string();
            loop {
                let control = reader.read_decimal(1);
                binary += &reader.read_binary(4);
                if control == 0 {
                    return (
                        Packet {
                            version,
                            type_id,
                            payload: Payload::Literal(binary_to_decimal(&binary).unwrap()),
                        },
                        reader,
                    );
                }
            }
        }
        _ => {
            let mut subpackets = vec![];
            match reader.read_decimal(1) {
                0 => {
                    let subpacket_length = reader.read_decimal(15) as usize;
                    let ending = reader.position + subpacket_length;
                    loop {
                        let (packet, new_reader) = parse_packet(reader);
                        reader = new_reader;
                        subpackets.push(packet);
                        if reader.position >= ending {
                            return (
                                Packet {
                                    version,
                                    type_id,
                                    payload: Payload::Operator(subpackets),
                                },
                                reader,
                            );
                        }
                    }
                }
                1 => {
                    let subpacket_count = reader.read_decimal(11);
                    for _ in 0..subpacket_count {
                        let (packet, new_reader) = parse_packet(reader);
                        reader = new_reader;
                        subpackets.push(packet);
                    }
                    (
                        Packet {
                            version,
                            type_id,
                            payload: Payload::Operator(subpackets),
                        },
                        reader,
                    )
                }
                _ => panic!("invalid length id"),
            }
        }
    }
}

fn parse(input: &str) -> Packet {
    let reader = HexReader::new(input.to_string());
    let (packet, _) = parse_packet(reader);
    packet
}

fn main() {
    println!("Part One: {}", parse(INPUT).version_sum());
    println!("Part Two: {}", parse(INPUT).apply());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_conversions() {
        assert_eq!(5, binary_to_decimal("0101").unwrap());
        assert_eq!(
            "11101110000000001101010000001100100000100011000001100000",
            hex_to_binary("EE00D40C823060").unwrap()
        );
        assert_eq!(
            "101000000000000101101100100010000000000101100010000000010111110000110110100001101011000110001010001111010100011110000000",
            hex_to_binary("A0016C880162017C3686B18A3D4780").unwrap()
        );
    }

    fn assert_literal(packet: &Packet, version: u64, type_id: u64, value: u64) {
        assert_eq!(version, packet.version);
        assert_eq!(type_id, packet.type_id);
        match packet.payload {
            Payload::Literal(actual_value) => assert_eq!(value, actual_value),
            _ => panic!("unexpected payload"),
        }
    }

    #[test]
    fn test_hex_reader() {
        let mut reader = HexReader::new("EE00D40C823060".to_string());
        assert_eq!(7, reader.read_decimal(3));
        assert_eq!(3, reader.read_decimal(3));
        assert_eq!(1, reader.read_decimal(1));
        assert_eq!(3, reader.read_decimal(11));
    }

    #[test]
    fn test_literal_parsing() {
        let reader = HexReader::new("D2FE28".to_string());
        let (packet, _) = parse_packet(reader);
        assert_literal(&packet, 6, 4, 2021);
    }

    #[test]
    fn test_operator_fifeteen() {
        let reader = HexReader::new("38006F45291200".to_string());
        let (packet, _) = parse_packet(reader);
        assert_eq!(1, packet.version);
        assert_eq!(6, packet.type_id);
        match packet.payload {
            Payload::Operator(values) => {
                assert_eq!(2, values.len());
                assert_literal(values.get(0).unwrap(), 6, 4, 10);
                assert_literal(values.get(1).unwrap(), 2, 4, 20);
            }
            _ => panic!("unexpected payload"),
        }
    }

    #[test]
    fn test_operator_eleven() {
        let reader = HexReader::new("EE00D40C823060".to_string());
        let (packet, _) = parse_packet(reader);

        assert_eq!(7, packet.version);
        assert_eq!(3, packet.type_id);
        match packet.payload {
            Payload::Operator(values) => {
                assert_literal(values.get(0).unwrap(), 2, 4, 1);
                assert_literal(values.get(1).unwrap(), 4, 4, 2);
                assert_literal(values.get(2).unwrap(), 1, 4, 3);
            }
            _ => panic!("unexpected payload"),
        }
    }

    #[test]
    fn test_packet_construction() {
        let a = Packet {
            version: 1,
            type_id: 4,
            payload: Payload::Literal(3),
        };

        let b = Packet {
            version: 3,
            type_id: 4,
            payload: Payload::Literal(3),
        };

        assert_eq!(3, b.version_sum());

        let c = Packet {
            version: 2,
            type_id: 6,
            payload: Payload::Operator(vec![a, b]),
        };

        assert_eq!(6, c.version_sum());
    }

    #[test]
    fn test_part_one_example_input() {
        assert_eq!(16, parse("8A004A801A8002F478").version_sum());
        assert_eq!(12, parse("620080001611562C8802118E34").version_sum());
        assert_eq!(23, parse("C0015000016115A2E0802F182340").version_sum());
        assert_eq!(31, parse("A0016C880162017C3686B18A3D4780").version_sum());
    }

    #[test]
    fn test_part_two_example_input() {
        assert_eq!(3, parse("C200B40A82").apply());
        assert_eq!(54, parse("04005AC33890").apply());
        assert_eq!(7, parse("880086C3E88112").apply());
        assert_eq!(9, parse("CE00C43D881120").apply());
        assert_eq!(1, parse("D8005AC2A8F0").apply());
        assert_eq!(0, parse("F600BC2D8F").apply());
        assert_eq!(0, parse("9C005AC2F8F0").apply());
        assert_eq!(1, parse("9C0141080250320F1802104A08").apply());
    }
}
