use anyhow::Result;
use nom::{
    bits::{bits, complete::take},
    character::complete::one_of,
    combinator::map,
    error::Error,
    multi::{many1, many_m_n},
    sequence::tuple,
    IResult,
};

use crate::Runner;

#[derive(Debug, Clone, PartialEq)]
pub enum PacketType {
    Literal { value: u64 },
    Sum { sub: Vec<Packet> },
    Product { sub: Vec<Packet> },
    Minimum { sub: Vec<Packet> },
    Maximum { sub: Vec<Packet> },
    GreaterThan { sub: Vec<Packet> },
    LessThan { sub: Vec<Packet> },
    Equal { sub: Vec<Packet> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Packet {
    version: usize,
    t: PacketType,
}

impl Packet {
    pub fn version(&self) -> usize {
        self.version
            + match &self.t {
                PacketType::Literal { .. } => 0,
                PacketType::Sum { sub }
                | PacketType::Product { sub }
                | PacketType::Minimum { sub }
                | PacketType::Maximum { sub }
                | PacketType::GreaterThan { sub }
                | PacketType::LessThan { sub }
                | PacketType::Equal { sub } => sub.iter().map(|p| p.version()).sum(),
            }
    }

    pub fn value(&self) -> u64 {
        match &self.t {
            PacketType::Literal { value } => *value,
            PacketType::Sum { sub } => sub.iter().fold(0, |a, p| a + p.value()),
            PacketType::Product { sub } => sub.iter().fold(1, |a, p| a * p.value()),
            PacketType::Minimum { sub } => sub
                .iter()
                .fold(u64::MAX, |a, p| std::cmp::min(a, p.value())),
            PacketType::Maximum { sub } => sub.iter().fold(0, |a, p| std::cmp::max(a, p.value())),
            PacketType::GreaterThan { sub } => {
                assert_eq!(2, sub.len());
                if sub[0].value() > sub[1].value() {
                    1
                } else {
                    0
                }
            }
            PacketType::LessThan { sub } => {
                assert_eq!(2, sub.len());
                if sub[0].value() < sub[1].value() {
                    1
                } else {
                    0
                }
            }
            PacketType::Equal { sub } => {
                assert_eq!(2, sub.len());
                if sub[0].value() == sub[1].value() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

pub struct Day;

impl Runner<usize, u64> for Day {
    type Input = Vec<Packet>;

    fn day() -> usize {
        16
    }

    fn get_input<'a>(input: &'a str) -> Result<Self::Input> {
        let (input, bytes) = bytes(input.trim())?;
        assert_eq!("", input);

        let (bytes, packets) =
            // bits::<_, _, Error<BitStream<'a>>, Error<&'a [u8]>, _>(
                packets(&bytes).unwrap();
        assert_eq!(b"", bytes);
        Ok(packets)
    }

    fn part1(input: &Self::Input) -> Result<usize> {
        Ok(input.iter().map(|p| p.version()).sum())
    }

    fn part2(input: &Self::Input) -> Result<u64> {
        Ok(input.iter().map(|p| p.value()).sum())
    }
}

fn bytes<'a>(input: &'a str) -> IResult<&'a str, Vec<u8>, ()> {
    many1(map(many_m_n(2, 2, one_of("0123456789ABCDEF")), |s| {
        u8::from_str_radix(&s.into_iter().collect::<String>(), 16).unwrap()
    }))(input)
}

type BitStream<'a> = (&'a [u8], usize);

fn packets<'a>(input: &'a [u8]) -> IResult<&'a [u8], Vec<Packet>, Error<&'a [u8]>> {
    Ok(bits(many1(packet))(input)?)
}

fn packet<'a>(input: BitStream<'a>) -> IResult<BitStream<'a>, Packet, Error<BitStream<'a>>> {
    let (input, (version, id)): (_, (usize, u8)) = tuple((take(3usize), take(3usize)))(input)?;
    let (input, p) = match id {
        4 => literal(version)(input)?,
        id => operator(version, id)(input)?,
    };
    Ok((input, p))
}

fn literal<'a>(
    version: usize,
) -> impl Fn(BitStream<'a>) -> IResult<BitStream<'a>, Packet, Error<BitStream<'a>>> {
    move |mut input: BitStream| {
        let mut value: u64 = 0;
        loop {
            let (rest, (nonce, chunk)): (_, (u8, u64)) =
                tuple((take(1usize), take(4usize)))(input)?;
            value <<= 4;
            value += chunk;

            if nonce == 0u8 {
                return Ok((
                    rest,
                    Packet {
                        version,
                        t: PacketType::Literal { value },
                    },
                ));
            }
            input = rest;
        }
    }
}

fn operator<'a>(
    version: usize,
    id: u8,
) -> impl Fn(BitStream<'a>) -> IResult<BitStream<'a>, Packet, Error<BitStream<'a>>> {
    move |input: BitStream| {
        let (input, length_type) = take(1usize)(input)?;
        let (input, sub) = match length_type {
            // Length in bits
            0usize => {
                let (input, length) = take(15usize)(input)?;
                fixed_length(length)(input)?
            }
            _ => {
                let (input, num) = take(11usize)(input)?;
                sub_packets(num)(input)?
            }
        };
        Ok((
            input,
            Packet {
                version,
                t: match id {
                    0 => PacketType::Sum { sub },
                    1 => PacketType::Product { sub },
                    2 => PacketType::Minimum { sub },
                    3 => PacketType::Maximum { sub },
                    4 => unreachable!(),
                    5 => PacketType::GreaterThan { sub },
                    6 => PacketType::LessThan { sub },
                    7 => PacketType::Equal { sub },
                    _ => unreachable!(),
                },
            },
        ))
    }
}

fn fixed_length<'a>(
    length: usize,
) -> impl Fn(BitStream<'a>) -> IResult<BitStream<'a>, Vec<Packet>, Error<BitStream<'a>>> {
    move |mut input: BitStream| {
        let total_length = input.0.len() * 8 - input.1;
        let mut read = 0;
        let mut packets = vec![];
        while read < length {
            let (rest, p) = packet(input)?;
            read = total_length - (rest.0.len() * 8 - rest.1);
            packets.push(p);

            input = rest;
        }
        assert_eq!(read, length);
        Ok((input, packets))
    }
}

fn sub_packets<'a>(
    number: usize,
) -> impl Fn(BitStream<'a>) -> IResult<BitStream<'a>, Vec<Packet>, Error<BitStream<'a>>> {
    move |input: BitStream| many_m_n(number, number, packet)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "8A004A801A8002F478";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(16, Day::part1(&input)?);
        Ok(())
    }

    #[test]
    fn sample2() -> Result<()> {
        let input = "620080001611562C8802118E34";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(12, Day::part1(&input)?);
        Ok(())
    }

    #[test]
    fn sample3() -> Result<()> {
        let input = "C0015000016115A2E0802F182340";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(23, Day::part1(&input)?);
        Ok(())
    }

    #[test]
    fn sample4() -> Result<()> {
        let input = "A0016C880162017C3686B18A3D4780";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(31, Day::part1(&input)?);
        Ok(())
    }

    #[test]
    fn sample5() -> Result<()> {
        let input = "D2FE28";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(
            Packet {
                version: 6,
                t: PacketType::Literal { value: 2021 },
            },
            input[0]
        );
        assert_eq!(6, Day::part1(&input)?);
        Ok(())
    }

    #[test]
    fn part2_1() -> Result<()> {
        let input = "C200B40A82";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(3, Day::part2(&input)?);
        Ok(())
    }

    #[test]
    fn part2_2() -> Result<()> {
        let input = "04005AC33890";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(54, Day::part2(&input)?);
        Ok(())
    }
}
