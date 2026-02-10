use std::cell::RefCell;
use chrono::NaiveDate;
use rand::Rng;
use std::f64::consts::PI;
use crate::Flight;

fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const EARTH_RADIUS_MILES: f64 = 3959.0;
    let dlat = (lat2 - lat1) * PI / 180.0;
    let dlon = (lon2 - lon1) * PI / 180.0;
    let lat1_rad = lat1 * PI / 180.0;
    let lat2_rad = lat2 * PI / 180.0;

    let a = (dlat / 2.0).sin().powi(2)
        + lat1_rad.cos() * lat2_rad.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();

    EARTH_RADIUS_MILES * c
}

pub fn generate_random_flights(n: usize) -> Vec<Flight> {
    let airports: Vec<(&str, f64, f64)> = vec![
        ("ATL", 33.640411, -84.419853),
        ("ORD", 41.978611, -87.904724),
        ("LAX", 33.942791, -118.410042),
        ("DFW", 32.897480, -97.040443),
        ("DEN", 39.849312, -104.673828),
        ("JFK", 40.641766, -73.780968),
        ("SFO", 37.615223, -122.389977),
        ("SEA", 47.443546, -122.301659),
        ("LAS", 36.086010, -115.153969),
        ("MCO", 28.424618, -81.310753),
        ("CLT", 35.213890, -80.943054),
        ("PHX", 33.435249, -112.010216),
        ("MIA", 25.795160, -80.279594),
        ("IAH", 29.993067, -95.341812),
        ("EWR", 40.689491, -74.174538),
        ("BOS", 42.365589, -71.010025),
        ("DTW", 42.213249, -83.352859),
        ("PHL", 39.872940, -75.243988),
        ("LGA", 40.776863, -73.874069),
        ("FLL", 26.074215, -80.150726),
        ("BWI", 39.177540, -76.668526),
        ("SAN", 32.732346, -117.196053),
        ("SJC", 37.363949, -121.928940),
        ("DAL", 32.848152, -96.851349),
        ("BNA", 36.131687, -86.668823),
        ("TPA", 27.979168, -82.539337),
        ("MKE", 42.949890, -87.900414),
        ("CVG", 39.053276, -84.663017),
        ("SNA", 33.678925, -117.862869),
        ("TUS", 32.116112, -110.941109),
        ("ROC", 43.128002, -77.665474),
        ("AVL", 35.436077, -82.541298),
        ("TYS", 35.805813, -83.989815),
        ("MEM", 35.040031, -89.981873),
        ("ABQ", 35.0402, -106.609),
        ("HOU", 29.6454, -95.2789),
        ("MDW", 41.786, -87.7524),
        ("PDX", 45.5887, -122.5933),
    ];

    let mut rng = rand::thread_rng();
    let mut flights = Vec::with_capacity(10000);

    for _ in 0..n {
        let orig_idx = rng.gen_range(0..airports.len());
        let mut dest_idx = rng.gen_range(0..airports.len());
        while dest_idx == orig_idx {
            dest_idx = rng.gen_range(0..airports.len());
        }

        let (orig_code, orig_lat, orig_lon) = airports[orig_idx];
        let (dest_code, dest_lat, dest_lon) = airports[dest_idx];

        let mileage = haversine_distance(orig_lat, orig_lon, dest_lat, dest_lon).round() as u32;

        let number = rng.gen_range(100..9999);

        let month = rng.gen_range(1..=12);
        let day = rng.gen_range(1..=28);
        let dep_date = NaiveDate::from_ymd_opt(2026, month, day).unwrap();

        let cancelled = rng.gen_bool(0.05);

        let gate = if rng.gen_bool(0.8) {
            let has_prefix = rng.gen_bool(0.5);
            let prefix = if has_prefix {
                (rng.gen_range(b'A'..=b'Z') as char).to_string()
            } else {
                String::new()
            };
            let num = rng.gen_range(1..=99);
            Some(format!("{}{}", prefix, num))
        } else {
            None
        };

        flights.push(Flight {
            number,
            orig: orig_code.to_string(),
            dest: dest_code.to_string(),
            dep_date,
            mileage,
            cancelled: RefCell::new(cancelled),
            gate: RefCell::new(gate),
        });
    }

    flights
}