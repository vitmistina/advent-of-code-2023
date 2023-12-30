use super::*;

impl Stats {
    fn identify_quadrant(&mut self, tile: (&Coordinate, &Tile), map: &HashMap<Coordinate, Tile>) {
        if let Some(prev) = match (tile.0.x, tile.0.y) {
            (1, 2) => map.get(&Coordinate { x: 1, y: 1 }),
            (1, -2) => map.get(&Coordinate { x: 1, y: -1 }),
            (-1, 2) => map.get(&Coordinate { x: -1, y: 1 }),
            (-1, -2) => map.get(&Coordinate { x: -1, y: -1 }),
            (_, _) => None,
        } {
            self.quadrants.push(TileStatistics {
                position: tile.0.clone(),
                tile: prev.clone(),
                repeats_every: tile.1.iteration_started - prev.iteration_started,
            })
        };
    }
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn identifies_quadrants() {
        let map = HashMap::from([
            (
                Coordinate { x: 1, y: 1 },
                Tile {
                    starting: Coordinate { x: 0, y: 0 },
                    iteration_started: 15,
                },
            ),
            (
                Coordinate { x: -1, y: -2 },
                Tile {
                    starting: Coordinate { x: 0, y: 0 },
                    iteration_started: 11,
                },
            ),
        ]);

        let coord = Coordinate { x: 1, y: 2 };
        let tile = Tile {
            starting: Coordinate { x: 0, y: 0 },
            iteration_started: 26,
        };

        let mut stats = Stats {
            horizontals: Vec::new(),
            quadrants: Vec::new(),
        };

        stats.identify_quadrant((&coord, &tile), &map);

        assert_eq!(stats.quadrants.len(), 1);

        assert_eq!(
            stats.quadrants[0],
            TileStatistics {
                position: coord.clone(),
                tile: Tile {
                    starting: Coordinate { x: 0, y: 0 },
                    iteration_started: 15,
                },
                repeats_every: 11
            }
        );

        let coord = Coordinate { x: 2, y: 1 };
        let tile = Tile {
            starting: Coordinate { x: 0, y: 0 },
            iteration_started: 26,
        };
        stats.identify_quadrant((&coord, &tile), &map);
        assert_eq!(stats.quadrants.len(), 1);
    }
}
