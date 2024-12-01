use std::collections::HashMap;

use anyhow::Result;
use itertools::{join, Itertools};

use crate::Runner;

pub struct Day04;

pub struct Day04Slow;

#[derive(Default, Debug, Clone)]
pub struct Passport {
    birth_year: Option<usize>,
    issue_year: Option<usize>,
    expire_year: Option<usize>,
    height: Option<Height>,
    hair_color: Option<HairColor>,
    eye_color: Option<EyeColor>,
    passport_id: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Height {
    Cm(usize),
    In(usize),
    Wrong,
}

impl Height {
    fn from(s: &str) -> Option<Self> {
        let len = &s[..s.len() - 2];
        let unit = &s[s.len() - 2..];
        match unit {
            "cm" => Some(Self::Cm(len.parse().unwrap_or(0))),
            "in" => Some(Self::In(len.parse().unwrap_or(0))),
            _ => Some(Self::Wrong),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum EyeColor {
    Correct,
    Wrong,
}

impl EyeColor {
    fn from(s: &str) -> Self {
        match s {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => Self::Correct,
            _ => Self::Wrong,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum HairColor {
    Correct,
    Wrong,
}

impl HairColor {
    fn from(s: &str) -> Self {
        if s.starts_with('#') && s.len() == 7 && s.chars().skip(1).all(|c| c.is_ascii_hexdigit()) {
            Self::Correct
        } else {
            Self::Wrong
        }
    }
}

impl Passport {
    fn new(line: &str) -> Option<Self> {
        let mut this = Self::default();
        line.split_whitespace().for_each(|entry| {
            let entry = entry.split(':').collect::<Vec<_>>();
            let (k, v) = (entry[0], entry[1]);
            match k {
                "byr" => this.birth_year = Some(v.parse().unwrap_or(0)),
                "iyr" => this.issue_year = Some(v.parse().unwrap_or(0)),
                "eyr" => this.expire_year = Some(v.parse().unwrap_or(0)),
                "hcl" => this.hair_color = Some(HairColor::from(v)),
                "ecl" => this.eye_color = Some(EyeColor::from(v)),
                "pid" => {
                    this.passport_id = Some(v.len() == 9 && v.chars().all(|c| c.is_ascii_digit()))
                }
                "hgt" => this.height = Height::from(v),
                _ => (),
            }
        });
        Some(this)
    }

    fn contains_all(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expire_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }
    fn is_valid(&self) -> bool {
        (1920..=2002).contains(&self.birth_year.unwrap_or(0))
            && (2010..=2020).contains(&self.issue_year.unwrap_or(0))
            && (2020..=2030).contains(&self.expire_year.unwrap_or(0))
            && self.hair_color == Some(HairColor::Correct)
            && self.eye_color == Some(EyeColor::Correct)
            && match self.height {
                Some(Height::Cm(i)) => (150..=193).contains(&i),
                Some(Height::In(i)) => (59..=76).contains(&i),
                _ => false,
            }
            && self.passport_id == Some(true)
    }
}

impl Runner for Day04 {
    type Input = Vec<Passport>;
    type Output = usize;

    fn day() -> usize {
        4
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        Ok(input
            .lines()
            .group_by(|l| l.is_empty())
            .into_iter()
            .map(|(_, l)| join(l, " "))
            .filter_map(|l| Passport::new(&l))
            .collect())
    }

    fn part1(input: &Self::Input) -> Result<usize> {
        Ok(input.iter().filter(|p| p.contains_all()).count())
    }

    fn part2(input: &Self::Input) -> Result<usize> {
        Ok(input.iter().filter(|p| p.is_valid()).count())
    }
}

impl Runner for Day04Slow {
    type Input = Vec<HashMap<String, String>>;
    type Output = usize;

    fn day() -> usize {
        4
    }

    fn comment() -> &'static str {
        "Slow"
    }

    fn get_input(input: &str) -> Result<Vec<HashMap<String, String>>> {
        let entries = input
            .split("\n\n")
            .map(|entry| {
                entry
                    .split_whitespace()
                    .flat_map(|p| p.split(':'))
                    .map(&str::to_owned)
                    .tuples()
                    .collect()
            })
            .collect::<Vec<HashMap<_, _>>>();
        Ok(entries)
    }

    fn part1(input: &Vec<HashMap<String, String>>) -> Result<usize> {
        Ok(input
            .iter()
            .filter(|pass| {
                ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                    .iter()
                    .all(|k| pass.contains_key(*k))
            })
            .count())
    }

    fn part2(input: &Vec<HashMap<String, String>>) -> Result<usize> {
        Ok(input
            .iter()
            .filter(|pass| {
                ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                    .iter()
                    .all(|k| pass.contains_key(*k))
            })
            .filter(|pass| {
                pass.iter().all(|(k, v)| match k.as_str() {
                    "byr" => (1920..=2002).contains(&v.parse().unwrap_or(0)),
                    "iyr" => (2010..=2020).contains(&v.parse().unwrap_or(0)),
                    "eyr" => (2020..=2030).contains(&v.parse().unwrap_or(0)),
                    "hcl" => {
                        v.starts_with('#')
                            && v.len() == 7
                            && v.chars().skip(1).all(|c| c.is_ascii_hexdigit())
                    }
                    "ecl" => {
                        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v.as_str())
                    }
                    "pid" => v.len() == 9 && v.chars().all(|c| c.is_ascii_digit()),
                    "cid" => true,
                    "hgt" => {
                        let height = v[0..(v.len() - 2)].parse().unwrap_or(0);
                        match &v[(v.len() - 2)..] {
                            "cm" => (150..=193).contains(&height),
                            "in" => (59..=76).contains(&height),
                            _ => false,
                        }
                    }
                    _ => unreachable!(),
                })
            })
            .count())
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn sample1() -> Result<()> {
    //     let input = get_input(
    //         "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
    //          byr:1937 iyr:2017 cid:147 hgt:183cm\n\
    //          \n\
    //          iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
    //          hcl:#cfa07d byr:1929\n\
    //          \n\
    //          hcl:#ae17e1 iyr:2013\n\
    //          eyr:2024\n\
    //          ecl:brn pid:760753108 byr:1931\n\
    //          hgt:179cm\n\
    //          \n\
    //          hcl:#cfa07d eyr:2025 pid:166559648\n\
    //          iyr:2011 ecl:brn hgt:59in",
    //     )
    //     .unwrap();
    //     assert_eq!(2, part1(&input)?);
    //     assert_eq!(2, part2(&input)?);
    //     Ok(())
    // }

    // #[test]
    // fn invalid() -> Result<()> {
    //     let input = "eyr:1972 cid:100\n\
    //                     hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\
    //                     \n\
    //                     iyr:2019\n\
    //                     hcl:#602927 eyr:1967 hgt:170cm\n\
    //                     ecl:grn pid:012533040 byr:1946\n\
    //                     \n\
    //                     hcl:dab227 iyr:2012\n\
    //                     ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\
    //                     \n\
    //                     hgt:59cm ecl:zzz\n\
    //                     eyr:2038 hcl:74454a iyr:2023\n\
    //                     pid:3556412378 byr:2007 ";
    //     let map = get_input(input).unwrap();
    //     let pass = get_input_struct(input).unwrap();
    //     assert_eq!(0, part2(&map)?);
    //     assert_eq!(0, part2_struct(&pass)?);
    //     Ok(())
    // }

    // #[test]
    // fn valid() -> Result<()> {
    //     let input = get_input(
    //         "pid:087499704 hgt:73in ecl:grn iyr:2012 eyr:2030 byr:1980\n\
    //          hcl:#623a2f\n\
    //          \n\
    //          eyr:2029 ecl:blu cid:129 byr:1989\n\
    //          iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\
    //          \n\
    //          hcl:#888785\n\
    //          hgt:164cm byr:2001 iyr:2015 cid:88\n\
    //          pid:545766238 ecl:hzl\n\
    //          eyr:2022\n\
    //          \n\
    //          iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
    //     )
    //     .unwrap();
    //     assert_eq!(4, part2(&input)?);
    //     Ok(())
    // }
}
