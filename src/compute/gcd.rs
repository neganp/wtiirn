use crate::model::{Coordinates};

pub type Meters = f64;

const EARTH_RADIUS: Meters = 6_371_000.0;

/// Verify that a number is in [-1, 1], making a small allowance for floating
/// point errors.
fn check_acos_domain(x: f64) -> f64 {
    const ACCEPTABLE_DELTA: f64 = 1e-11;
    if -1.0 <= x && x <= 1.0 {
        return x
    }
    if x < -1.0 && (-1.0 - ACCEPTABLE_DELTA) < x {
        return -1.0 + std::f64::EPSILON;
    }
    if x > 1.0 && (1.0 + ACCEPTABLE_DELTA) > x {
        return 1.0 - std::f64::EPSILON;
    }
    panic!("f64 outside of arcos range ({})", x);
}

pub fn great_circle_distance(p1: &Coordinates, p2: &Coordinates) -> Meters {
    // https://en.wikipedia.org/wiki/Great-circle_distance
    let (lat1, lon1) = p1.to_radians();
    let (lat2, lon2) = p2.to_radians();
    let delta_lon = (lon1 - lon2).abs();
    let cos_central = lat1.sin() * lat2.sin() + lat1.cos() * lat2.cos() * delta_lon.cos();
    println!("cos_central = {}", cos_central);
    let central = check_acos_domain(cos_central).acos();
    println!("central = {}", central);
    central * EARTH_RADIUS
}

#[cfg(test)]
mod test {
    use super::*;

    const ACCEPTABLE_ERR: Meters = 0.33;
    const EARTH_CIRC: Meters = 2.0 * std::f64::consts::PI * EARTH_RADIUS;

    #[test]
    fn distance_to_self_is_zero() {
        let p1 = Coordinates {
            lat: 100.0,
            lon: 45.0,
        };
        assert!(great_circle_distance(&p1, &p1) < ACCEPTABLE_ERR);
    }

    #[test]
    fn distance_from_equator_to_pole_is_one_quarter_circumference() {
        let p1 = Coordinates { lat: 0.0, lon: 0.0 };
        let p2 = Coordinates {
            lat: 90.0,
            lon: 0.0,
        };
        let dist = great_circle_distance(&p1, &p2);
        assert!((dist - EARTH_CIRC / 4.0).abs() < ACCEPTABLE_ERR);
    }
    #[test]
    fn distance_from_pole_to_pole_is_one_half_circumference() {
        let p1 = Coordinates {
            lat: -90.0,
            lon: 0.0,
        };
        let p2 = Coordinates {
            lat: 90.0,
            lon: 0.0,
        };
        let dist = great_circle_distance(&p1, &p2);
        assert!((dist - EARTH_CIRC / 2.0).abs() < ACCEPTABLE_ERR);
    }

    #[test]
    fn quarter_circumference_centered_on_equator() {
        // Check that the distance form 45 N to 45 S is a quarter circumference.
        let p1 = Coordinates {
            lat: 45.0,
            lon: 0.0,
        };
        let p2 = Coordinates {
            lat: -45.0,
            lon: 0.0,
        };
        let dist = great_circle_distance(&p1, &p2);
        assert!((dist - EARTH_CIRC / 4.0).abs() < ACCEPTABLE_ERR);
    }
    #[test]
    fn two_half_turns() {
        //Check a point halfway around the world
        let p1 = Coordinates {
            lat: 45.0,
            lon: 0.0,
        };
        let p2 = Coordinates {
            lat: -45.0,
            lon: 180.0,
        };
        let dist = great_circle_distance(&p1, &p2);
        println!("{} {}", dist, EARTH_CIRC / 2.0);
        assert!((dist - EARTH_CIRC / 2.0).abs() < ACCEPTABLE_ERR);
    }
}
