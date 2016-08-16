#[derive(Debug,PartialEq,PartialOrd)]
pub struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
}

impl Coordinate {
    pub fn new() -> Result<Self, &'static str> {
        Ok(Coordinate { x: 0, y: 0, z: 0 })
    }

    pub fn at(x: i64, y: i64, z: i64) -> Result<Self, &'static str> {
        if x + y + z == 0 {
            Ok(Coordinate { x: x, y: y, z: z })
        } else {
            Err("Invalid cubic coordinates")
        }
    }

    pub fn neighbors(&self) -> Vec<Coordinate> {
        vec![
            Coordinate::at(self.x + 1, self.y, self.z - 1).unwrap(),
            Coordinate::at(self.x + 1, self.y - 1, self.z).unwrap(),
            Coordinate::at(self.x - 1, self.y + 1, self.z).unwrap(),
            Coordinate::at(self.x - 1, self.y, self.z + 1).unwrap(),
            Coordinate::at(self.x, self.y + 1, self.z - 1).unwrap(),
            Coordinate::at(self.x, self.y - 1, self.z + 1).unwrap(),
        ]
    }

    pub fn distance_to(&self, other: Coordinate) -> i64 {
        (
            (self.x - other.x).abs()
            + (self.y - other.y).abs()
            + (self.z - other.z).abs()
        ) / 2
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_makes_a_new_one() {
        let coord = Coordinate::new().unwrap();
        assert_eq!(coord.x, 0);
        assert_eq!(coord.y, 0);
        assert_eq!(coord.z, 0);
    }

    #[test]
    fn it_accepts_args_in_constructor() {
        let coord = Coordinate::at(-3, -1, 4).unwrap();
        assert_eq!(coord.x, -3);
        assert_eq!(coord.y, -1);
        assert_eq!(coord.z, 4);
    }

    #[test]
    fn it_rejects_invalid_cube_coordinates() {
        let coord = Coordinate::at(3, 1, 4);
        assert!(coord.is_err());
    }

    #[test]
    fn it_generates_a_list_of_neighbors() {
        let coord = Coordinate::new().unwrap();
        let expected = vec![
            Coordinate::at(1, 0, -1).unwrap(),
            Coordinate::at(1, -1, 0).unwrap(),
            Coordinate::at(-1, 1, 0).unwrap(),
            Coordinate::at(-1, 0, 1).unwrap(),
            Coordinate::at(0, 1, -1).unwrap(),
            Coordinate::at(0, -1, 1).unwrap(),
        ];
        assert_eq!(coord.neighbors(), expected);
    }

    #[test]
    fn it_calcuates_distances() {
        let coord_a = Coordinate::at(-3, -1, 4).unwrap();
        let coord_b = Coordinate::at(2, 7, -9).unwrap();
        assert_eq!(coord_a.distance_to(coord_b), 13);
    }
}
