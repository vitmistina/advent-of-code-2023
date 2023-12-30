use super::*;
impl Stats {
    fn identify_horizontal(&mut self, tile: (&Coordinate, &Tile), garden: &Garden) {
        if tile.0.x != 0 && tile.0.y != 0 {
            return;
        }
        let neighbors = tile.0.get_neighbors_with_wrapping();
        for n in neighbors {
            if let Some(prev) = garden.tiles.get(&n) {
                if prev.starting == tile.1.starting && self.is_missing_from_horizontals(tile.0) {
                    let tile_stat = TileStatistics {
                        position: n.clone(),
                        tile: prev.clone(),
                        repeats_every: tile.1.iteration_started - prev.iteration_started,
                    };
                    self.horizontals.push((tile_stat, HashSet::new()))
                }
            };
        }
    }

    fn is_missing_from_horizontals(&self, tile_coord: &Coordinate) -> bool {
        match (tile_coord.x, tile_coord.y) {
            (0, _) => {
                self.horizontals
                    .iter()
                    .any(|(stat, _)| stat.position.x == 0)
                    == false
            }
            (_, 0) => {
                self.horizontals
                    .iter()
                    .any(|(stat, _)| stat.position.y == 0)
                    == false
            }
            (_, _) => false,
        }
    }
}

impl Garden {
    fn get_snapshot(&self, tile_coord: &Coordinate) -> HashSet<Coordinate> {
        self.steps
            .iter()
            .filter_map(|step| Some(step.clone()))
            .collect()
    }
}

struct StepBounds {
    x_min: usize,
    x_max: usize,
    y_min: usize,
    y_max: usize,
}

fn get_step_bounds(tile_coord: &Coordinate) -> StepBounds {
    todo!()
}

#[cfg(test)]
mod t {

    use super::*;

    #[test]
    fn identifies_horizontal() {
        let garden = Garden {
            rocks: HashSet::new(),
            steps: HashSet::from([
                Coordinate { x: 9, y: 0 },
                Coordinate { x: 10, y: 0 },
                Coordinate { x: 3, y: -6 },
            ]),
            y_size: 3,
            x_size: 3,
            tiles: HashMap::from([
                (
                    Coordinate { x: 2, y: 0 },
                    Tile {
                        starting: Coordinate { x: 0, y: 0 },
                        iteration_started: 21,
                    },
                ),
                (
                    Coordinate { x: 0, y: -2 },
                    Tile {
                        starting: Coordinate { x: 10, y: 10 },
                        iteration_started: 21,
                    },
                ),
            ]),
        };

        let coord = Coordinate { x: 3, y: 0 };
        let tile = Tile {
            starting: Coordinate { x: 0, y: 0 },
            iteration_started: 32,
        };

        let mut stats = Stats {
            horizontals: Vec::new(),
            quadrants: Vec::new(),
        };

        stats.identify_horizontal((&coord, &tile), &garden);

        assert_eq!(stats.horizontals.len(), 1);
        assert_eq!(
            stats.horizontals[0].0,
            TileStatistics {
                position: Coordinate { x: 2, y: 0 },
                tile: Tile {
                    starting: Coordinate { x: 0, y: 0 },
                    iteration_started: 21,
                },
                repeats_every: 11
            }
        );
        assert_eq!(
            stats.horizontals[0].1,
            HashSet::from([Coordinate { x: 3, y: 0 }, Coordinate { x: 4, y: 0 }])
        );

        stats.identify_horizontal((&coord, &tile), &garden);

        assert_eq!(stats.horizontals.len(), 1);
    }
}
