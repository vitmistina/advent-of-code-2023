use super::*;
impl Stats {
    pub(crate) fn identify_horizontal(
        &mut self,
        tile_tuple: (&Coordinate, &Tile),
        garden: &Garden,
    ) {
        let coord = tile_tuple.0.clone();
        let tile = tile_tuple.1.clone();

        if coord.x != 0 && coord.y != 0 {
            return;
        }
        let neighbors = coord.get_neighbors_with_wrapping();
        for n in neighbors {
            if let Some(prev) = garden.tiles.get(&n) {
                if prev.starting == tile.starting && self.is_missing_from_horizontals(&coord) {
                    let tile_stat = TileStatistics {
                        position: coord.clone(),
                        tile: prev.clone(),
                        repeats_every: tile.iteration_started - prev.iteration_started,
                        snapshots: HashMap::new(),
                    };
                    self.horizontals.push((tile_stat))
                }
            };
        }
    }

    fn is_missing_from_horizontals(&self, tile_coord: &Coordinate) -> bool {
        match (tile_coord.x, tile_coord.y) {
            (0, _) => {
                self.horizontals
                    .iter()
                    .filter(|stat| stat.position.x == 0)
                    .count()
                    < 2
            }
            (_, 0) => {
                self.horizontals
                    .iter()
                    .filter(|stat| stat.position.y == 0)
                    .count()
                    < 2
            }
            (_, _) => false,
        }
    }
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
            stats.horizontals[0],
            TileStatistics {
                position: Coordinate { x: 3, y: 0 },
                tile: Tile {
                    starting: Coordinate { x: 0, y: 0 },
                    iteration_started: 21,
                },
                repeats_every: 11,
                snapshots: HashMap::new()
            }
        );

        stats.identify_horizontal((&coord, &tile), &garden);

        assert_eq!(stats.horizontals.len(), 2);
        
        stats.identify_horizontal((&coord, &tile), &garden);

        assert_eq!(stats.horizontals.len(), 2);
    }
}
