use std::{collections::HashMap, time::Instant};

use crate::utils::grid2d::Grid2D;

// personal functions
//use crate::utils::grid2d;
// use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 20 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day20_input_demo1.txt");
    let input = include_str!("../assets/day20_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

#[derive(Clone, PartialEq, Eq, Debug)]
struct Zone {
    grid: Grid2D,
    borders: Vec<Vec<char>>,
    neighb: [Option<usize>; 4], // top, bottom, left, right
    flip_vert: bool,
    rotate: usize,
    is_corner: bool,
}

fn get_answer(input: &str) -> Option<usize> {
    let mut zones = get_zones_from_input_file(input);

    let mut result = 0;
    if let Some(a_corner) = find_all_neighbours(&mut zones) {
        let mut big_grid = construct_map(&mut zones, a_corner);
        let monsters = find_sea_monsters(&mut big_grid);
        result = big_grid.count_occurences('#') - monsters * 15;
        return Some(result);
    }
    Some(result)
}

fn find_sea_monsters(big_grid: &mut Grid2D) -> usize {
    let mut sea_monster = Grid2D::new_empty(3, 20, ' ');
    sea_monster.set_at((0, 18), '#');
    sea_monster.set_at((1, 0), '#');
    sea_monster.set_at((1, 5), '#');
    sea_monster.set_at((1, 6), '#');
    sea_monster.set_at((1, 11), '#');
    sea_monster.set_at((1, 12), '#');
    sea_monster.set_at((1, 17), '#');
    sea_monster.set_at((1, 18), '#');
    sea_monster.set_at((1, 19), '#');
    sea_monster.set_at((2, 1), '#');
    sea_monster.set_at((2, 4), '#');
    sea_monster.set_at((2, 7), '#');
    sea_monster.set_at((2, 10), '#');
    sea_monster.set_at((2, 13), '#');
    sea_monster.set_at((2, 16), '#');
    // sea_monster.print();
    let mut max_found = 0;

    let a_big_grid = big_grid.clone();
    let monsters = count_sea_monsters(a_big_grid, sea_monster.clone());
    max_found = max_found.max(monsters);

    // println!("rotating");
    let big_grid2 = big_grid.clone().rotate();
    let monsters = count_sea_monsters(big_grid2, sea_monster.clone());
    max_found = max_found.max(monsters);

    // println!("rotating");
    let big_grid2 = big_grid.clone().rotate().rotate();
    let monsters = count_sea_monsters(big_grid2, sea_monster.clone());
    max_found = max_found.max(monsters);

    // println!("rotating");
    let big_grid2 = big_grid.clone().rotate().rotate().rotate();
    let monsters = count_sea_monsters(big_grid2, sea_monster.clone());
    max_found = max_found.max(monsters);

    // println!("flipping");
    let big_grid2 = big_grid.clone().flip_vertical();
    let monsters = count_sea_monsters(big_grid2, sea_monster.clone());
    max_found = max_found.max(monsters);

    // println!("rotating");
    let big_grid2 = big_grid.clone().flip_vertical().rotate();
    let monsters = count_sea_monsters(big_grid2, sea_monster.clone());
    max_found = max_found.max(monsters);

    // println!("rotating");
    let big_grid2 = big_grid.clone().flip_vertical().rotate().rotate();
    let monsters = count_sea_monsters(big_grid2, sea_monster.clone());
    max_found = max_found.max(monsters);

    // println!("rotating");
    let big_grid2 = big_grid.clone().flip_vertical().rotate().rotate().rotate();
    let monsters = count_sea_monsters(big_grid2, sea_monster.clone());
    max_found = max_found.max(monsters);

    // println!("max sea monsters is {max_found}");

    max_found
}

fn count_sea_monsters(big_grid: Grid2D, sea_monster: Grid2D) -> usize {
    let mut monsters = 0;
    let max_l = big_grid.max_l;
    let max_c = big_grid.max_c;
    for start_l in 0..max_l - sea_monster.max_l {
        for start_c in 0..max_c - sea_monster.max_c {
            let mut found = 0;
            for l in 0..sea_monster.max_l {
                for c in 0..sea_monster.max_c {
                    if sea_monster.get_at((l, c)) == '#'
                        && big_grid.get_at((l + start_l, c + start_c)) == '#'
                    {
                        found += 1;
                    }
                }
            }
            if found == 15 {
                monsters += 1;
                // println!("found one sea monster");
            }
        }
    }
    monsters
}

#[derive(Debug)]
enum MaskType {
    None,
    Id,
    Some,
}
#[derive(Debug)]
struct Mask {
    mask: MaskType,
    val: usize,
}

fn construct_map(zones: &mut HashMap<usize, Zone>, a_corner: usize) -> Grid2D {
    let square_size = (zones.len() as f32).sqrt() as usize;

    let grid_width = 10;

    let mut big_grid = Grid2D::new_empty(
        grid_width * square_size + 1,
        grid_width * square_size + 1,
        '.',
    );

    // map = grid of the zone ids
    let mut map: Vec<Vec<usize>> = Vec::new();
    let mut map_row = Vec::new();
    for l in 0..square_size {
        for c in 0..square_size {
            // first corner
            if l == 0 && c == 0 {
                map_row.push(a_corner);
                // borders should be bottom (1) & right (3)
                // println!("map({l},{c}) is {}", a_corner);
                if let Some(tile) = zones.get_mut(&a_corner) {
                    set_orientation(
                        tile,
                        [
                            Mask {
                                mask: MaskType::None,
                                val: 0,
                            },
                            Mask {
                                mask: MaskType::Some,
                                val: 0,
                            },
                            Mask {
                                mask: MaskType::None,
                                val: 0,
                            },
                            Mask {
                                mask: MaskType::Some,
                                val: 0,
                            },
                        ],
                    );
                    add_grid(&mut big_grid, &tile.grid, l, c);
                }
            } else {
                let mut mask = [
                    Mask {
                        mask: MaskType::None,
                        val: 0,
                    },
                    Mask {
                        mask: MaskType::Some,
                        val: 0,
                    },
                    Mask {
                        mask: MaskType::None,
                        val: 0,
                    },
                    Mask {
                        mask: MaskType::Some,
                        val: 0,
                    },
                ];
                if l != 0 {
                    mask[0] = Mask {
                        mask: MaskType::Id,
                        val: map[l - 1][c],
                    };
                }
                if l == square_size - 1 {
                    mask[1] = Mask {
                        mask: MaskType::None,
                        val: 0,
                    };
                }
                if c != 0 {
                    mask[2] = Mask {
                        mask: MaskType::Id,
                        val: *map_row.last().unwrap(),
                    };
                }
                if c == square_size - 1 {
                    mask[3] = Mask {
                        mask: MaskType::None,
                        val: 0,
                    };
                }

                let mut left_id = None;
                let mut down_id = None;
                let mut tile_id = None;
                if c != 0 {
                    let right = map_row.last().unwrap();
                    if let Some(right_tile) = zones.clone().get(right) {
                        if let Some(left_neighb) = right_tile.neighb[3] {
                            // println!("map({l},{c}) is {}", left_neighb);
                            left_id = Some(left_neighb);
                            tile_id = left_id;
                        }
                    }
                }
                if l != 0 {
                    let top = map[l - 1][c];
                    if let Some(top_tile) = zones.clone().get(&top) {
                        if let Some(down_neighb) = top_tile.neighb[1] {
                            // println!("map({l},{c}) is {}", down_neighb);
                            down_id = Some(down_neighb);
                            tile_id = down_id;
                        }
                    }
                }
                if left_id.is_some() && down_id.is_some() {
                    assert_eq!(left_id, down_id);
                }
                map_row.push(tile_id.unwrap());
                if let Some(tile) = zones.get_mut(&tile_id.unwrap()) {
                    set_orientation(tile, mask);
                    add_grid(&mut big_grid, &tile.grid, l, c);
                }
            }
        }
        map.push(map_row.clone());
        map_row = Vec::new();
    }
    // println!("map : {:?}", map);
    // big_grid.print();
    big_grid
}

fn add_grid(big_grid: &mut Grid2D, tile: &Grid2D, l: usize, c: usize) {
    let grid_width = 8;
    big_grid.insert_tile(
        &tile.get_tile(1, 1, grid_width, grid_width),
        l * grid_width,
        c * grid_width,
    );
}

fn is_good_orientation(tile: &mut Zone, mask: &[Mask; 4]) -> bool {
    let neigh = tile.neighb;
    for idx in 0..4 {
        if (matches!(mask[idx].mask, MaskType::None) && neigh[idx].is_some())
            || (matches!(mask[idx].mask, MaskType::Some) && neigh[idx].is_none())
            || (matches!(mask[idx].mask, MaskType::Id)
                && !(neigh[idx].is_some() && mask[idx].val == neigh[idx].unwrap()))
        {
            return false;
        }
    }
    true
}

fn set_orientation(tile: &mut Zone, mask: [Mask; 4]) {
    // println!("orientation must be {:?}",mask);
    if !is_good_orientation(tile, &mask) {
        tile.rotate = 90;
        straighten_tile(tile);
        if !is_good_orientation(tile, &mask) {
            tile.rotate = 90;
            straighten_tile(tile);
            if !is_good_orientation(tile, &mask) {
                tile.rotate = 90;
                straighten_tile(tile);
                if !is_good_orientation(tile, &mask) {
                    tile.rotate = 90;
                    tile.flip_vert = true;
                    straighten_tile(tile);
                    if !is_good_orientation(tile, &mask) {
                        tile.rotate = 90;
                        straighten_tile(tile);
                        if !is_good_orientation(tile, &mask) {
                            tile.rotate = 90;
                            straighten_tile(tile);
                            if !is_good_orientation(tile, &mask) {
                                tile.rotate = 90;
                                straighten_tile(tile);
                            }
                        }
                    }
                }
            }
        }
    }
    if !is_good_orientation(tile, &mask) {
        println!("problem with tile {:?}", tile.neighb);
        println!("can't find orientation that fits {:?}", mask);
        panic!();
    }
    // println!("neightb are {:?}",tile.neighb);
}

fn straighten_tile(tile: &mut Zone) {
    if tile.flip_vert {
        let grid = tile.grid.clone();
        let new_grid = grid.flip_vertical();
        tile.grid = new_grid;
        tile.neighb.swap(0, 1);
        tile.borders[2].reverse();
        tile.borders[3].reverse();
        tile.borders.swap(0, 1);
        tile.flip_vert = false;
    }
    let angle = tile.rotate / 90;
    for _ in 0..angle {
        let grid = tile.grid.clone();
        let new_grid = grid.rotate(); // counter-clockwise
        tile.grid = new_grid;
        tile.rotate = 0;
    }
    match angle {
        1 => {
            tile.neighb.swap(0, 2);
            tile.neighb.swap(1, 3);
            tile.neighb.swap(0, 1);
        }
        2 => {
            tile.neighb.swap(0, 1);
            tile.neighb.swap(2, 3);
        }
        3 => {
            tile.neighb.swap(1, 2);
            tile.neighb.swap(0, 3);
            tile.neighb.swap(0, 1);
        }
        _ => {}
    }
    tile.borders = vec![
        tile.grid.get_line(0),                     // top
        tile.grid.get_line(tile.grid.max_l - 1),   // bottom
        tile.grid.get_column(0),                   // left
        tile.grid.get_column(tile.grid.max_c - 1), // right
    ];
}

fn find_all_neighbours(zones: &mut HashMap<usize, Zone>) -> Option<usize> {
    let mut a_corner = None;
    let zones_ids = zones.iter().map(|(&id, _)| id).collect::<Vec<usize>>();

    // find possible neighbours
    // println!("there are {} zones", zones.len());
    let zones_clone = zones.clone();
    for id in &zones_ids {
        if let Some(z) = zones.get_mut(id) {
            let mut neighbours = [None; 4];
            // println!("zone {}, {:?}",z.id, z.borders);
            for (i, b) in z.borders.iter().enumerate() {
                for id2 in &zones_ids {
                    if let Some(z2) = zones_clone.get(id2) {
                        if id != id2 {
                            for b2 in &z2.borders {
                                if b == b2 {
                                    // println!("border {i} of {} = border {i2} of {}", id, id2);
                                    neighbours[i] = Some(*id2);
                                    // println!("{:?}",neighbours);
                                }
                                let mut b_rev = b.clone();
                                b_rev.reverse();
                                if b_rev == *b2 {
                                    // println!("border {i} of {} = REV border {i2} of {}", id, id2);
                                    neighbours[i] = Some(*id2);
                                    // println!("{:?}",neighbours);
                                }
                            }
                        }
                    }
                }
            }
            // println!("neighb of {} : {:?}",z.id,z.neighb);
            z.neighb = neighbours;
            if a_corner.is_none()
                && neighbours
                    .iter()
                    .filter(|x| x.is_some())
                    .collect::<Vec<_>>()
                    .len()
                    == 2
            {
                a_corner = Some(*id);
            }
        }
    }
    // for z in zones.iter() {
    //     println!("{} neighb = {:?}", z.0, z.1.neighb);
    // }

    a_corner
}

fn get_zones_from_input_file(input: &str) -> HashMap<usize, Zone> {
    let mut zones = HashMap::new();
    let parts = input.split("\n\n").collect::<Vec<_>>();
    for p in parts {
        let id = p
            .lines()
            .take(1)
            .map(|line| {
                line.split([' ', ':'])
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect::<Vec<usize>>()[0]
            })
            .collect::<Vec<_>>()[0];
        let grid_str = p.lines().skip(1).collect::<Vec<_>>().join("\n");
        let grid = Grid2D::new(&grid_str);

        // grid.print();
        let borders = vec![
            grid.get_line(0),                // top
            grid.get_line(grid.max_l - 1),   // bottom
            grid.get_column(0),              // left
            grid.get_column(grid.max_c - 1), // right
        ];
        zones.insert(
            id,
            Zone {
                grid,
                borders,
                neighb: [None; 4],
                flip_vert: false,
                rotate: 0,
                is_corner: false,
            },
        );
    }
    zones
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_straighten() {
        let mut zones = HashMap::new();
        let grid = Grid2D::new("12\n34");
        zones.insert(
            1,
            Zone {
                grid,
                borders: vec![
                    vec!['1', '2'],
                    vec!['3', '4'],
                    vec!['1', '3'],
                    vec!['2', '4'],
                ],
                neighb: [Some(0), Some(1), Some(2), Some(3)],
                flip_vert: false,
                rotate: 0,
                is_corner: false,
            },
        );

        // test vertical flip
        let mut zones_copy = zones.clone();
        let tile_copy = zones_copy.get_mut(&1).unwrap();
        tile_copy.flip_vert = true;
        straighten_tile(tile_copy);
        let new_grid = Grid2D::new("34\n12");
        assert_eq!(
            *tile_copy,
            Zone {
                grid: new_grid,
                neighb: [Some(1), Some(0), Some(2), Some(3)],
                borders: vec![
                    vec!['3', '4'],
                    vec!['1', '2'],
                    vec!['3', '1'],
                    vec!['4', '2'],
                ],
                flip_vert: false,
                rotate: 0,
                is_corner: false,
            }
        );

        // test rotation 90° counter-clockwise
        let mut zones_copy = zones.clone();
        let tile_copy = zones_copy.get_mut(&1).unwrap();
        tile_copy.rotate = 90;
        straighten_tile(tile_copy);
        let new_grid = Grid2D::new("24\n13");
        assert_eq!(
            *tile_copy,
            Zone {
                grid: new_grid,
                neighb: [Some(3), Some(2), Some(0), Some(1)],
                borders: vec![
                    vec!['2', '4'],
                    vec!['1', '3'],
                    vec!['2', '1'],
                    vec!['4', '3'],
                ],
                flip_vert: false,
                rotate: 0,
                is_corner: false,
            }
        );
    }

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day20_input_demo1.txt")),
            Some(273)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day20_input.txt")),
            Some(1901)
        );
    }
}
