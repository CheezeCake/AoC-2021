use std::io;

#[derive(Debug)]
struct Packet {
    version: usize,
    type_id: usize,
    val: Value,
}

#[derive(Debug)]
enum Value {
    Literal(usize),
    Operator(Vec<Packet>),
}

#[derive(Debug)]
struct Parser {
    s: Vec<u8>,
    pos: usize,
}

impl Parser {
    fn new(s: Vec<u8>) -> Self {
        Self { s, pos: 0 }
    }

    fn next(&mut self) -> u8 {
        let b = self.s[self.pos];
        self.pos += 1;
        b
    }

    fn parse_packet(&mut self) -> Packet {
        let version = self.parse_version();
        let type_id = self.parse_type_id();
        let val = if type_id == 4 {
            Value::Literal(self.parse_literal())
        } else {
            let lenght_type_id = self.next();
            let mut sub_packets = Vec::new();

            if lenght_type_id == 0 {
                let sub_packets_len = self.read_number(15);
                let end = self.pos + sub_packets_len;
                while self.pos < end {
                    sub_packets.push(self.parse_packet());
                }
            } else {
                let sub_packets_cnt = self.read_number(11);
                for _ in 0..sub_packets_cnt {
                    sub_packets.push(self.parse_packet());
                }
            }
            Value::Operator(sub_packets)
        };

        Packet {
            version,
            type_id,
            val,
        }
    }

    fn read_number(&mut self, bits: usize) -> usize {
        let mut n: usize = 0;
        for _ in 0..bits {
            n = (n << 1) | (self.next() as usize);
        }
        n
    }

    fn parse_version(&mut self) -> usize {
        self.read_number(3)
    }

    fn parse_type_id(&mut self) -> usize {
        self.read_number(3)
    }

    fn parse_literal(&mut self) -> usize {
        let mut literal = 0;
        loop {
            let prefix = self.next();
            literal = (literal << 4) | self.read_number(4);
            if prefix == 0 {
                return literal;
            }
        }
    }
}

impl Packet {
    fn version_sum(&self) -> usize {
        self.version
            + match &self.val {
                Value::Literal(_) => 0,
                Value::Operator(sub_packets) => {
                    sub_packets.iter().map(|packet| packet.version_sum()).sum()
                }
            }
    }

    fn evaluate(&self) -> usize {
        match &self.val {
            Value::Literal(n) => *n,
            Value::Operator(sub_packets) => match self.type_id {
                0 => sub_packets.iter().map(|p| p.evaluate()).sum(),
                1 => sub_packets.iter().map(|p| p.evaluate()).product(),
                2 => sub_packets.iter().map(|p| p.evaluate()).min().unwrap(),
                3 => sub_packets.iter().map(|p| p.evaluate()).max().unwrap(),
                5 => {
                    if sub_packets[0].evaluate() > sub_packets[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if sub_packets[0].evaluate() < sub_packets[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if sub_packets[0].evaluate() == sub_packets[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("invalid type_id: {}", self.type_id),
            },
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: Vec<u8> = input
        .trim()
        .bytes()
        .map(|hex_digit| {
            format!(
                "{:04b}",
                match hex_digit {
                    b'0'..=b'9' => hex_digit - b'0',
                    b'A'..=b'F' => hex_digit - b'A' + 10,
                    b'a'..=b'f' => hex_digit - b'a' + 10,
                    _ => panic!("invalid hex digit: {}", hex_digit),
                }
            )
            .bytes()
            .map(|b| b - b'0')
            .collect::<Vec<u8>>()
        })
        .flatten()
        .collect();

    let mut parser = Parser::new(input);
    let packet = parser.parse_packet();

    println!("part 1: {}", packet.version_sum());
    println!("part 2: {}", packet.evaluate());
}
