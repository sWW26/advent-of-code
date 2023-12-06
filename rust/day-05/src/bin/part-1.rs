use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("input1.txt");

    let answer = solve(input);

    println!("{}", answer);
}

#[derive(Debug)]
struct MapKey(RangeInclusive<u64>);

impl Eq for MapKey {}

impl PartialEq<Self> for MapKey {
    fn eq(&self, other: &Self) -> bool {
        (self.0.start() <= other.0.start() && self.0.end() >= other.0.end())
            || (self.0.start() >= other.0.start() && self.0.end() <= other.0.end())
    }
}

impl PartialOrd<Self> for MapKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.eq(&other) {
            Some(Ordering::Equal)
        } else if self.0.start() > other.0.end() {
            Some(Ordering::Greater)
        } else if self.0.end() < other.0.start() {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

impl Ord for MapKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).expect("ranges cannot overlap")
    }
}

fn build_map_for_block<'a>(lines: &mut impl Iterator<Item = &'a str>) -> BTreeMap<MapKey, u64> {
    let mut map = BTreeMap::new();
    let mut end_of_block = false;
    while !end_of_block {
        let l = lines.next();
        match l {
            Some("") | None => end_of_block = true,
            Some(x) => {
                let mut parts = x.split(' ');
                let to = parts
                    .next()
                    .expect("should be three parts")
                    .parse::<u64>()
                    .expect("map to should always be a number");
                let from = parts
                    .next()
                    .expect("should be three parts")
                    .parse::<u64>()
                    .expect("map from should always be a number");
                let count = parts
                    .next()
                    .expect("should be three parts")
                    .parse::<u64>()
                    .expect("map count should always be a number");
                assert_eq!(parts.next(), None);
                map.insert(MapKey(from..=(from + count - 1)), to);
            }
        }
    }
    map
}

fn solve(str: &str) -> u64 {
    let mut lines = str.lines();
    let seeds_line = lines.next().expect("should always have a line");
    // seeds: 79 14 55 13
    let seeds = seeds_line
        .split_once(':')
        .expect("first line starts with 'seeds:'")
        .1
        .split(' ')
        .filter_map(|x| {
            if x.is_empty() {
                None
            } else {
                Some(x.parse::<u64>().expect("seeds must be numbers"))
            }
        });

    // skip empty line
    let _ = lines.next();

    let heading = lines.next();
    assert_eq!(heading, Some("seed-to-soil map:"));

    let seed_to_soil_map = build_map_for_block(&mut lines);

    let heading = lines.next();
    assert_eq!(heading, Some("soil-to-fertilizer map:"));
    let soil_to_fertilizer_map = build_map_for_block(&mut lines);

    let heading = lines.next();
    assert_eq!(heading, Some("fertilizer-to-water map:"));
    let fertilizer_to_water_map = build_map_for_block(&mut lines);

    let heading = lines.next();
    assert_eq!(heading, Some("water-to-light map:"));
    let water_to_light_map = build_map_for_block(&mut lines);

    let heading = lines.next();
    assert_eq!(heading, Some("light-to-temperature map:"));
    let light_to_temperature_map = build_map_for_block(&mut lines);

    let heading = lines.next();
    assert_eq!(heading, Some("temperature-to-humidity map:"));
    let temperature_to_humidity_map = build_map_for_block(&mut lines);

    let heading = lines.next();
    assert_eq!(heading, Some("humidity-to-location map:"));
    let humidity_to_location_map = build_map_for_block(&mut lines);

    let maps = [
        seed_to_soil_map,
        soil_to_fertilizer_map,
        fertilizer_to_water_map,
        water_to_light_map,
        light_to_temperature_map,
        temperature_to_humidity_map,
        humidity_to_location_map,
    ];

    seeds
        .map(|mut s| {
            for map in &maps {
                s = map
                    .get_key_value(&MapKey(s..=s))
                    // x.1 = start of to range
                    // x.0.0 = from range
                    // s = exact item mapping from
                    .map(|x| x.1 + (s - x.0 .0.start()))
                    .unwrap_or(s);
            }
            s
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = "seeds: 79 14 55 13

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

        let answer = solve(input);

        assert_eq!(answer, 35);
    }
}
