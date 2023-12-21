use once_cell::sync::Lazy;
use winnow::{
    ascii::digit1,
    combinator::{alt, preceded, repeat, separated, separated_pair, terminated},
    PResult, Parser,
};

pub static TEST_DATA: Lazy<Vec<&str>> = Lazy::new(|| {
    vec![
        "seeds: 79 14 55 13",
        "",
        "seed-to-soil map:",
        "50 98 2",
        "52 50 48",
        "",
        "soil-to-fertilizer map:",
        "0 15 37",
        "37 52 2",
        "39 0 15",
        "",
        "fertilizer-to-water map:",
        "49 53 8",
        "0 11 42",
        "42 0 7",
        "57 7 4",
        "",
        "water-to-light map:",
        "88 18 7",
        "18 25 70",
        "",
        "light-to-temperature map:",
        "45 77 23",
        "81 45 19",
        "68 64 13",
        "",
        "temperature-to-humidity map:",
        "0 69 1",
        "1 0 69",
        "",
        "humidity-to-location map:",
        "60 56 37",
        "56 93 4",
        "",
    ]
});

pub static TEST_DATA_STR: &str = r#"seeds: 79 14 55 13

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
56 93 4"#;

use super::{Layer, Mapping};

pub(super) fn parse_seeds(input: &mut &str) -> PResult<Vec<u64>> {
    terminated(preceded("seeds: ", parse_list), "\n\n").parse_next(input)
}

pub(super) fn parse_all_maps(input: &mut &str) -> PResult<Vec<Layer>> {
    separated(1.., parse_map, "\n").parse_next(input)
}

fn parse_digits(input: &mut &str) -> PResult<u64> {
    digit1.parse_to().parse_next(input)
}

fn parse_map_entry(input: &mut &str) -> PResult<Mapping> {
    terminated(parse_list, "\n")
        .verify(|values: &[u64]| values.len() == 3)
        .map(|values: Vec<u64>| Mapping {
            destination: values[0],
            source: values[1],
            range: values[2],
        })
        .parse_next(input)
}

fn parse_list(input: &mut &str) -> PResult<Vec<u64>> {
    separated(1.., parse_digits, ' ').parse_next(input)
}

fn parse_map(input: &mut &str) -> PResult<Layer> {
    preceded(parse_mapping_header, repeat(1.., parse_map_entry))
        .map(|maps| Layer(maps))
        .parse_next(input)
}

fn parse_map_label<'s>(input: &mut &'s str) -> PResult<&'s str> {
    alt((
        "seed",
        "soil",
        "fertilizer",
        "water",
        "light",
        "temperature",
        "humidity",
        "location",
    ))
    .parse_next(input)
}

fn parse_mapping_header<'s>(input: &mut &'s str) -> PResult<(&'s str, &'s str)> {
    terminated(
        separated_pair(parse_map_label, "-to-", parse_map_label),
        " map:\n",
    )
    .parse_next(input)
}

#[test]
fn test_winnow_parsing() {
    let mut test_data = TEST_DATA_STR;

    let seeds = parse_seeds.parse_next(&mut test_data).unwrap();

    assert_eq!(seeds, vec![79, 14, 55, 13]);

    let all_mappings = parse_all_maps(&mut test_data).unwrap();

    assert_eq!(7, all_mappings.len());
    assert_eq!(
        Mapping {
            destination: 60,
            source: 56,
            range: 37
        },
        all_mappings
            .last()
            .and_then(|last| last.0.last())
            .unwrap()
            .clone()
    );
}
