use std::{
    collections::{BTreeMap, HashMap},
    ops::Range,
};

use color_eyre::Result;
use winnow::{
    ascii::{digit1, line_ending, multispace0, space1},
    combinator::separated,
    PResult, Parser,
};

use crate::Runner;

#[derive(Debug, Clone, Default)]
struct RangeChanger {
    map: Vec<(Range<usize>, Range<usize>)>,
}

impl RangeChanger {
    fn get(&self, src: usize) -> usize {
        self.map
            .iter()
            .find(|(range, _)| range.contains(&src))
            .map(|(range, dst)| dst.start + (src - range.start))
            .unwrap_or(src)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Maps {
    seeds: Vec<usize>,
    seed_to_soil: RangeChanger,
    soil_to_fertilizer: RangeChanger,
    fertilizer_to_water: RangeChanger,
    water_to_light: RangeChanger,
    light_to_temperature: RangeChanger,
    temperature_to_humidity: RangeChanger,
    humidity_to_location: RangeChanger,
}

impl Maps {
    fn seed_to_location(&self, seed: usize) -> usize {
        let soil = self.seed_to_soil.get(seed);
        let fertilizer = self.soil_to_fertilizer.get(soil);
        let water = self.fertilizer_to_water.get(fertilizer);
        let light = self.water_to_light.get(water);
        let temperature = self.light_to_temperature.get(light);
        let humidity = self.temperature_to_humidity.get(temperature);
        self.humidity_to_location.get(humidity)
    }

    fn parse(input: &mut &str) -> PResult<Self> {
        let mut maps = Self::default();
        maps.seeds = (
            "seeds:",
            space1,
            separated(1.., digit1.try_map(str::parse::<usize>), space1),
        )
            .parse_next(input)?
            .2;
        let _ = (multispace0, "seed-to-soil map:", multispace0).parse_next(input)?;
        let lists = parse_map_list(input)?;
        mut_hash_map(lists, &mut maps.seed_to_soil);
        let _ = (multispace0, "soil-to-fertilizer map:", multispace0).parse_next(input)?;
        let lists = parse_map_list(input)?;
        mut_hash_map(lists, &mut maps.soil_to_fertilizer);
        let _ = (multispace0, "fertilizer-to-water map:", multispace0).parse_next(input)?;
        let lists = parse_map_list(input)?;
        mut_hash_map(lists, &mut maps.fertilizer_to_water);
        let _ = (multispace0, "water-to-light map:", multispace0).parse_next(input)?;
        let lists = parse_map_list(input)?;
        mut_hash_map(lists, &mut maps.water_to_light);
        let _ = (multispace0, "light-to-temperature map:", multispace0).parse_next(input)?;
        let lists = parse_map_list(input)?;
        mut_hash_map(lists, &mut maps.light_to_temperature);
        let _ = (multispace0, "temperature-to-humidity map:", multispace0).parse_next(input)?;
        let lists = parse_map_list(input)?;
        mut_hash_map(lists, &mut maps.temperature_to_humidity);
        let _ = (multispace0, "humidity-to-location map:", multispace0).parse_next(input)?;
        let lists = parse_map_list(input)?;
        mut_hash_map(lists, &mut maps.humidity_to_location);
        Ok(maps)
    }
}

fn mut_hash_map(lists: Vec<Vec<usize>>, maps: &mut RangeChanger) {
    lists.into_iter().for_each(|list| {
        let [dst, src, cnt] = list.as_slice() else {
            panic!("malformed map");
        };
        maps.map.push((*src..*src + *cnt, *dst..*dst + *cnt));
    });
}

fn parse_map_list(input: &mut &str) -> PResult<Vec<Vec<usize>>> {
    Ok(separated(
        1..,
        separated::<_, usize, Vec<usize>, _, _, _, _>(
            1..,
            digit1.try_map(str::parse::<usize>),
            space1,
        ),
        line_ending,
    )
    .parse_next(input)?)
}

pub struct Day;

impl Runner for Day {
    type Input<'input> = Maps;

    fn day() -> usize {
        5
    }

    fn get_input(mut input: &str) -> Result<Self::Input<'_>> {
        Ok(Maps::parse(&mut input).unwrap())
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        Ok(input
            .seeds
            .iter()
            .map(|seed| input.seed_to_location(*seed))
            .min()
            .unwrap_or(0))
    }

    // TODO: implement splitting of ranges, and binary-search(?)
    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        Ok(input
            .seeds
            .chunks_exact(2)
            .map(|slice| {
                (slice[0]..)
                    .take(slice[1])
                    .map(|seed| input.seed_to_location(seed))
            })
            .flatten()
            .min()
            .unwrap_or(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
            part1 = 35;
            part2 = 46;
    }

    prod_case! {
        part1 = 323142486;
        part2 = 201684;
    }
}
