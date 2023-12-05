use itertools::Itertools;
use std::{collections::HashMap, fs, path::Path};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum MapType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

fn name_to_map_type(name: &str) -> Option<MapType> {
    match name {
        "seed-to-soil" => Some(MapType::SeedToSoil),
        "soil-to-fertilizer" => Some(MapType::SoilToFertilizer),
        "fertilizer-to-water" => Some(MapType::FertilizerToWater),
        "water-to-light" => Some(MapType::WaterToLight),
        "light-to-temperature" => Some(MapType::LightToTemperature),
        "temperature-to-humidity" => Some(MapType::TemperatureToHumidity),
        "humidity-to-location" => Some(MapType::HumidityToLocation),
        _ => None,
    }
}

fn map(source: u64, map: &Vec<(u64, u64, u64)>) -> u64 {
    // Each map entry has the following values:
    // (destination_range_start, source_range_start, range_length)
    for (dest_start, source_start, length) in map {
        if *source_start <= source && source <= source_start + length - 1 {
            return dest_start + (source - source_start);
        }
    }
    source
}

pub fn problem1() {
    println!("problem 1");

    let path = Path::new("resources/day5_input");
    let contents = fs::read_to_string(path).unwrap();

    let mut seeds: Vec<u64> = Vec::new();
    let mut maps: HashMap<MapType, Vec<(u64, u64, u64)>> = HashMap::new();

    // Parse input file
    let mut current_map: Option<MapType> = None;
    contents.lines().into_iter().for_each(|line| {
        if seeds.is_empty() {
            let (_, numbers) = line.split_once(':').unwrap();
            seeds = numbers
                .trim()
                .split(' ')
                .into_iter()
                .map(|n| n.parse::<u64>().unwrap())
                .collect();
        } else if line.is_empty() {
            current_map = None;
        } else if current_map.is_none() {
            let (map_name, _) = line.split_once(' ').unwrap();
            current_map = Some(name_to_map_type(map_name).unwrap());
        } else {
            maps.entry(*current_map.as_ref().unwrap())
                .or_insert(Vec::new())
                .push(
                    line.trim()
                        .split(' ')
                        .map(|n| n.parse::<u64>().unwrap())
                        .collect_tuple()
                        .unwrap(),
                );
        }
    });

    let mut min_location = None;
    for seed in seeds {
        let soil = map(seed, maps.get(&MapType::SeedToSoil).unwrap());
        let fert = map(soil, maps.get(&MapType::SoilToFertilizer).unwrap());
        let watr = map(fert, maps.get(&MapType::FertilizerToWater).unwrap());
        let ligh = map(watr, maps.get(&MapType::WaterToLight).unwrap());
        let temp = map(ligh, maps.get(&MapType::LightToTemperature).unwrap());
        let humi = map(temp, maps.get(&MapType::TemperatureToHumidity).unwrap());
        let loca = map(humi, maps.get(&MapType::HumidityToLocation).unwrap());

        if min_location.is_none() || loca < min_location.unwrap() {
            min_location = Some(loca);
        }
    }

    println!("Answer: {}", min_location.unwrap());
}

pub fn problem2() {
    println!("problem 2");

    let path = Path::new("resources/day5_input");
    let contents = fs::read_to_string(path).unwrap();

    let mut seeds: Vec<(u64, u64)> = Vec::new();
    let mut maps: HashMap<MapType, Vec<(u64, u64, u64)>> = HashMap::new();

    // Parse input file
    let mut current_map: Option<MapType> = None;
    contents.lines().into_iter().for_each(|line| {
        if seeds.is_empty() {
            let (_, numbers) = line.split_once(':').unwrap();
            seeds = numbers
                .trim()
                .split(' ')
                .into_iter()
                .map(|n| n.parse::<u64>().unwrap())
                .chunks(2)
                .into_iter()
                .filter_map(|chunk| {
                    let mut chunk_iter = chunk.into_iter();
                    Some((chunk_iter.next()?, chunk_iter.next()?))
                })
                .collect::<Vec<(u64, u64)>>();
        } else if line.is_empty() {
            current_map = None;
        } else if current_map.is_none() {
            let (map_name, _) = line.split_once(' ').unwrap();
            current_map = Some(name_to_map_type(map_name).unwrap());
        } else {
            maps.entry(*current_map.as_ref().unwrap())
                .or_insert(Vec::new())
                .push(
                    line.trim()
                        .split(' ')
                        .map(|n| n.parse::<u64>().unwrap())
                        .collect_tuple()
                        .unwrap(),
                );
        }
    });

    // NOTE: This is the naive and VERY slow approach, using double for loop.
    // This would be done much better if the map function could be smarter and
    // handle a range of source values.

    let mut min_location = None;
    for (seed_start, seed_count) in seeds {
        for seed in seed_start..seed_start + seed_count + 1 {
            let soil = map(seed, maps.get(&MapType::SeedToSoil).unwrap());
            let fert = map(soil, maps.get(&MapType::SoilToFertilizer).unwrap());
            let watr = map(fert, maps.get(&MapType::FertilizerToWater).unwrap());
            let ligh = map(watr, maps.get(&MapType::WaterToLight).unwrap());
            let temp = map(ligh, maps.get(&MapType::LightToTemperature).unwrap());
            let humi = map(temp, maps.get(&MapType::TemperatureToHumidity).unwrap());
            let loca = map(humi, maps.get(&MapType::HumidityToLocation).unwrap());

            if min_location.is_none() || loca < min_location.unwrap() {
                min_location = Some(loca);
            }
        }
    }

    println!("Answer: {}", min_location.unwrap());
}
