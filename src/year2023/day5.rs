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

fn map2(sources: &Vec<(u64, u64)>, map: &Vec<(u64, u64, u64)>) -> Vec<(u64, u64)> {
    // Each map entry has the following values:
    // (destination_range_start, source_range_start, range_length)

    let mut destinations: Vec<(u64, u64)> = Vec::new();
    let mut sources_left = sources.clone();

    while let Some((source_first, source_length)) = sources_left.pop() {
        let source_last = source_first + source_length - 1;
        let mut found = false;
        for (map_dest_first, map_source_first, map_length) in map {
            let map_dest_last = *map_dest_first + map_length - 1;
            let map_source_last = map_source_first + map_length - 1;
            if *map_source_first <= source_first && source_last <= map_source_last {
                // Whole source range contained in this mapping. Can map to single destination range.
                destinations.push((
                    map_dest_first + (source_first - *map_source_first),
                    source_length,
                ));
                found = true;
                break;
            } else if *map_source_first <= source_first
                && source_first <= map_source_last
                && map_source_last < source_last
            {
                // First part of source range contained in this mapping, but not whole range.
                let dest_len = map_source_last - source_first + 1;
                destinations.push((map_dest_last - dest_len + 1, dest_len));
                sources_left.push((
                    map_source_last + 1,
                    source_length - (map_source_last - source_first + 1),
                ));
                found = true;
                break;
            } else if source_first < *map_source_first
                && *map_source_first <= source_last
                && source_last <= map_source_last
            {
                // Last part of source range contained in this mapping, but not whole range.
                destinations.push((*map_dest_first, source_last - *map_source_first + 1));
                sources_left.push((
                    source_first,
                    source_length - (source_last - *map_source_first + 1),
                ));
                found = true;
                break;
            }
        }
        if !found {
            // Whole source range is outside of any mapping range. Maps to itself.
            destinations.push((source_first, source_length));
        }
    }

    destinations
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

    let mut min_location = None;
    for seed_range in seeds {
        let seed = vec![seed_range];

        let soil = map2(&seed, maps.get(&MapType::SeedToSoil).unwrap());
        let fert = map2(&soil, maps.get(&MapType::SoilToFertilizer).unwrap());
        let watr = map2(&fert, maps.get(&MapType::FertilizerToWater).unwrap());
        let ligh = map2(&watr, maps.get(&MapType::WaterToLight).unwrap());
        let temp = map2(&ligh, maps.get(&MapType::LightToTemperature).unwrap());
        let humi = map2(&temp, maps.get(&MapType::TemperatureToHumidity).unwrap());
        let loca = map2(&humi, maps.get(&MapType::HumidityToLocation).unwrap());

        for (location_start, _) in loca {
            if min_location.is_none() || location_start < min_location.unwrap() {
                min_location = Some(location_start);
            }
        }
    }

    println!("Answer: {}", min_location.unwrap());
}
