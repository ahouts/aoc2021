use std::io::BufRead;

struct Biterator<'a> {
    data: &'a [u8],
    pos: usize,
    sub_pos: i8,
}

impl<'a> Iterator for Biterator<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let v = (*self.data.get(self.pos)? & (1 << self.sub_pos)) != 0;

        self.sub_pos -= 1;

        if self.sub_pos < 0 {
            self.pos += 1;
            self.sub_pos = 3;
        }

        Some(v)
    }
}

impl<'a> Biterator<'a> {
    fn new(data: &'a [u8]) -> Self {
        Biterator {
            data,
            pos: 0,
            sub_pos: 3,
        }
    }

    fn depth(&self) -> usize {
        self.pos * 4 + (4 - self.sub_pos as usize)
    }

    fn next_un_u8(&mut self, n: u8) -> Option<u8> {
        debug_assert!(n <= 8);

        let mut d = 0;
        for _ in 0..n {
            d <<= 1;
            if self.next()? {
                d += 1;
            }
        }
        Some(d)
    }

    fn next_un_u16(&mut self, n: u8) -> Option<u16> {
        debug_assert!(n <= 16);

        let mut d = 0;
        for _ in 0..n {
            d <<= 1;
            if self.next()? {
                d += 1;
            }
        }
        Some(d)
    }

    fn next_u3(&mut self) -> Option<u8> {
        self.next_un_u8(3)
    }

    fn next_u4(&mut self) -> Option<u8> {
        self.next_un_u8(4)
    }

    fn next_u11(&mut self) -> Option<u16> {
        self.next_un_u16(11)
    }

    fn next_u15(&mut self) -> Option<u16> {
        self.next_un_u16(15)
    }

    fn next_var_width_int(&mut self) -> Option<u64> {
        let mut res: u64 = 0;
        let mut last_iter = false;
        while !last_iter {
            last_iter = !self.next()?;
            res <<= 4;
            res += self.next_u4()? as u64;
        }
        Some(res)
    }
}

enum PacketData {
    Literal(u64),
    SubPackets(Vec<Packet>),
}

struct Packet {
    version: u8,
    type_id: u8,
    data: PacketData,
}

impl Packet {
    fn load(biterator: &mut Biterator) -> Option<Self> {
        let version = biterator.next_u3()?;
        let type_id = biterator.next_u3()?;
        let data = match type_id {
            4 => PacketData::Literal(biterator.next_var_width_int()?),
            _ => {
                if biterator.next()? {
                    let num_sub_packets = biterator.next_u11()?;
                    PacketData::SubPackets(
                        (0..num_sub_packets)
                            .map(|_| Packet::load(biterator))
                            .collect::<Option<Vec<Packet>>>()?,
                    )
                } else {
                    let sub_packet_length = biterator.next_u15()? as usize;
                    let depth = biterator.depth();
                    let mut sub_packets = Vec::new();
                    while biterator.depth() < depth + sub_packet_length {
                        sub_packets.push(Packet::load(biterator)?);
                    }
                    PacketData::SubPackets(sub_packets)
                }
            }
        };
        Some(Packet {
            version,
            type_id,
            data,
        })
    }

    fn version_sum(&self) -> u64 {
        self.version as u64
            + match &self.data {
                PacketData::Literal(_) => 0,
                PacketData::SubPackets(sub_packets) => sub_packets
                    .iter()
                    .map(|sub_packet| sub_packet.version_sum())
                    .sum(),
            }
    }

    fn eval(&self) -> u64 {
        match &self.data {
            PacketData::Literal(val) => *val,
            PacketData::SubPackets(sub_packets) => match self.type_id {
                0 => sub_packets.iter().map(|sub_packet| sub_packet.eval()).sum(),
                1 => sub_packets
                    .iter()
                    .map(|sub_packet| sub_packet.eval())
                    .product(),
                2 => sub_packets
                    .iter()
                    .map(|sub_packet| sub_packet.eval())
                    .min()
                    .unwrap(),
                3 => sub_packets
                    .iter()
                    .map(|sub_packet| sub_packet.eval())
                    .max()
                    .unwrap(),
                5 => {
                    if sub_packets[0].eval() > sub_packets[1].eval() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if sub_packets[0].eval() < sub_packets[1].eval() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if sub_packets[0].eval() == sub_packets[1].eval() {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!(),
            },
        }
    }
}

#[allow(dead_code)]
pub fn part1<R: BufRead>(reader: R) -> u64 {
    let line = reader.lines().next().unwrap().unwrap();
    let nibbles: Vec<u8> = line
        .bytes()
        .map(|c| {
            if (b'0'..=b'9').contains(&c) {
                c - b'0'
            } else {
                c - b'A' + 10
            }
        })
        .collect();

    let mut biterator = Biterator::new(nibbles.as_slice());
    let packet = Packet::load(&mut biterator).unwrap();
    packet.version_sum()
}

#[allow(dead_code)]
pub fn part2<R: BufRead>(reader: R) -> u64 {
    let line = reader.lines().next().unwrap().unwrap();
    let nibbles: Vec<u8> = line
        .bytes()
        .map(|c| {
            if (b'0'..=b'9').contains(&c) {
                c - b'0'
            } else {
                c - b'A' + 10
            }
        })
        .collect();

    let mut biterator = Biterator::new(nibbles.as_slice());
    let packet = Packet::load(&mut biterator).unwrap();
    packet.eval()
}
