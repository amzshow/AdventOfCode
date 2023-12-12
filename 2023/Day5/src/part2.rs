use indexmap::IndexMap;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Type {
    none,
    seed,
    soil,
    fertilizer,
    water,
    light,
    temperature,
    humidity,
    location,
}

struct Mapping {
    src_i: i64,
    src_j: i64,
    delta: i64,
}

fn parse_data_to_conversions(lines: std::str::Split<'_, &str>) -> IndexMap<Type, Type> {
    let mut conversions: IndexMap<Type, Type> = IndexMap::new();
    for line in lines {
        if line.contains("seed-to-soil map:") {
            conversions.insert(Type::seed, Type::soil);
        } else if line.contains("soil-to-fertilizer map:") {
            conversions.insert(Type::soil, Type::fertilizer);
        } else if line.contains("fertilizer-to-water map:") {
            conversions.insert(Type::fertilizer, Type::water);
        } else if line.contains("water-to-light map:") {
            conversions.insert(Type::water, Type::light);
        } else if line.contains("light-to-temperature map:") {
            conversions.insert(Type::light, Type::temperature);
        } else if line.contains("temperature-to-humidity map:") {
            conversions.insert(Type::temperature, Type::humidity);
        } else if line.contains("humidity-to-location map:") {
            conversions.insert(Type::humidity, Type::location);
        }
    }
    return conversions;
}

fn parse_data_to_type(lines: std::str::Split<'_, &str>) -> Vec<(Type, Vec<Vec<i64>>)> {
    let mut data: Vec<(Type, Vec<Vec<i64>>)> = vec![];
    let mut current: Type = Type::none;
    let mut numbers: Vec<Vec<i64>> = vec![];
    for line in lines {
        if line.contains("seeds:") {
            current = Type::none;
            data.push((
                Type::seed,
                vec![line
                    .replace("seeds: ", "")
                    .trim()
                    .split(" ")
                    .map(|x| x.parse::<i64>().expect(""))
                    .collect()],
            ));
        } else if line.contains("seed-to-soil map:") {
            current = Type::soil;
        } else if line.contains("soil-to-fertilizer map:") {
            current = Type::fertilizer;
        } else if line.contains("fertilizer-to-water map:") {
            current = Type::water;
        } else if line.contains("water-to-light map:") {
            current = Type::light;
        } else if line.contains("light-to-temperature map:") {
            current = Type::temperature;
        } else if line.contains("temperature-to-humidity map:") {
            current = Type::humidity;
        } else if line.contains("humidity-to-location map:") {
            current = Type::location;
        } else if line.find(char::is_numeric).is_some() {
            numbers.push(
                line.replace("seeds: ", "")
                    .trim()
                    .split(" ")
                    .map(|x| x.parse::<i64>().expect(""))
                    .collect(),
            );
        } else {
            if !matches!(current, Type::none) {
                let n: Vec<Vec<i64>> = numbers.clone();
                data.push((current, n));
                current = Type::none;
                numbers = vec![];
            }
        }
    }

    if !matches!(current, Type::none) {
        let n: Vec<Vec<i64>> = numbers.clone();
        data.push((current, n));
        current = Type::none;
        numbers = vec![];
    }

    return data;
}

fn calculate_source_to_locations_mapping(
    data: Vec<(Type, Vec<Vec<i64>>)>,
) -> HashMap<Type, Vec<Mapping>> {
    let mut locations_mapping: HashMap<Type, Vec<Mapping>> = HashMap::new();

    for i in 0..data.len() {
        let _type: &Type = &data[i].0;
        let numbers: &Vec<Vec<i64>> = &data[i].1;
        let mut mapping: Vec<Mapping> = vec![];

        for num in numbers {
            let source: i64 = num[1];
            let dest: i64 = num[0];
            let range: i64 = num[2];
            println!(
                "{} / {}: {} | {} -> {}",
                i + 1,
                data.len(),
                numbers.len(),
                source,
                dest
            );
            mapping.push(Mapping {
                src_i: source,
                src_j: source + range - 1,
                delta: dest - source,
            })
        }
        locations_mapping.insert(_type.clone(), mapping);
    }

    return locations_mapping;
}

fn convert_source_to_location(
    source: Vec<i64>,
    locations_mapping: HashMap<Type, Vec<Mapping>>,
    conversions: IndexMap<Type, Type>,
) -> HashMap<i64, i64> {
    let mut locations: HashMap<i64, i64> = HashMap::new();

    for si in 0..(source.len() / 2) {
        let start = source[si * 2];
        let end = source[si * 2] + source[si * 2 + 1];
        println!("{}/{} -> {}", si + 1, source.len() / 2, end - start);
        for _s in start..end {
            let mut s: i64 = _s.clone();
            for (_src_type, dest_type) in &conversions {
                let mappings: &Vec<Mapping> = locations_mapping.get(dest_type).expect("");
                for mapping in mappings {
                    if mapping.src_i <= s && s <= mapping.src_j {
                        s = s + mapping.delta;
                        break;
                    }
                }
            }
            locations.insert(_s, s);
        }
    }

    return locations;
}

pub fn call() {
    let content: String = fs::read_to_string("./input.txt")
        .expect("Read file")
        .replace("\r", "");

    let lines: std::str::Split<'_, &str> = content.split("\n");
    let _lines = lines.clone();

    let mut data: Vec<(Type, Vec<Vec<i64>>)> = parse_data_to_type(lines);
    let conversions: IndexMap<Type, Type> = parse_data_to_conversions(_lines);

    println!("Done Parsing");

    let source: Vec<i64> = data[0].1[0].clone();
    data.remove(0);

    let locations_mapping: HashMap<Type, Vec<Mapping>> =
        calculate_source_to_locations_mapping(data);

    println!("Done Mapping");

    let final_locations: HashMap<i64, i64> =
        convert_source_to_location(source, locations_mapping, conversions);

    println!("Done Src to Dest");

    println!("{:?}", final_locations);

    println!("{:?}", final_locations.values().min());

    // for s in source {
    //     if locations.contains_key(&s) {
    //         println!("{:?}: {:?}", s, locations.get(&s));
    //     } else {
    //         println!("{:?}: {:?}", s, s)
    //     }
    // }
}
