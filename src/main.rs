use csv::Writer;
use geo::{Point, HaversineDistance};
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use chrono::NaiveTime;
use itertools::Itertools;

mod data;

#[derive(Debug, Serialize)]
struct Agency {
    agency_id: String,
    agency_name: String,
    agency_url: String,
    agency_timezone: String,
}

#[derive(Debug, Serialize)]
struct Route {
    route_id: String,
    agency_id: String,
    route_short_name: String,
    route_long_name: String,
    route_type: u32,
    route_color: String,
    route_text_color: String,
}

#[derive(Debug, Serialize)]
struct Trip {
    route_id: String,
    service_id: String,
    trip_id: String,
    shape_id: String,
    block_id: String,
}

#[derive(Debug, Serialize)]
struct StopTime {
    trip_id: String,
    arrival_time: String,
    departure_time: String,
    stop_id: String,
    stop_sequence: u32,
    stop_headsign: String,
    timepoint: u8,
}

#[derive(Debug, Serialize)]
struct StopOutput {
    stop_id: String,
    stop_code: String,
    stop_name: String,
    stop_lat: f64,
    stop_lon: f64,
}

#[derive(Debug, Serialize)]
struct Calendar {
    service_id: String,
    monday: u8,
    tuesday: u8,
    wednesday: u8,
    thursday: u8,
    friday: u8,
    saturday: u8,
    sunday: u8,
    start_date: String,
    end_date: String,
}

#[derive(Debug, Serialize)]
struct CalendarDate {
    service_id: String,
    date: String,
    exception_type: u8, // 1 = Added, 2 = Removed
}

#[derive(Debug, serde::Deserialize)]
struct RawShape {
    shape_id: String,
    shape_pt_lat: f64,
    shape_pt_lon: f64,
    shape_pt_sequence: u32,
    #[serde(deserialize_with = "csv::invalid_option")]
    shape_dist_traveled: Option<f64>,
}

#[derive(Debug, serde::Deserialize)]
struct RawStop {
    stop_id: String,
    stop_name: String,
    stop_lat: f64,
    stop_lon: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    // 1. Load Input Data
    let mut stops_rdr = csv::Reader::from_path("input/stops.txt")?;
    let stops: Vec<RawStop> = stops_rdr.deserialize().collect::<Result<_, _>>()?;

    let mut shapes_rdr = csv::Reader::from_path("input/shapes.txt")?;
    let shapes: Vec<RawShape> = shapes_rdr.deserialize().collect::<Result<_, _>>()?;

    // 2. Load Stop Sequence from File
    let stop_seq_content = fs::read_to_string("input/stop_id_sequence.txt")?;
    let stop_sequence_ids: Vec<String> = stop_seq_content
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();

    if stop_sequence_ids.is_empty() {
        return Err("input/stop_id_sequence.txt is empty".into());
    }

    // Map stop_id to RawStop for easy lookup
    let stop_map: HashMap<String, &RawStop> = stops.iter()
        .map(|s| (s.stop_id.clone(), s))
        .collect();

    // 3. Prepare GTFS Output
    fs::create_dir_all("gtfs")?;

    // Agency
    let mut w = Writer::from_path("gtfs/agency.txt")?;
    w.serialize(Agency {
        agency_id: "IC".to_string(),
        agency_name: "Irvine Connect".to_string(),
        agency_url: "https://www.cityofirvine.org/irvine-connect".to_string(),
        agency_timezone: "America/Los_Angeles".to_string(),
    })?;

    // Calendar
    let mut w = Writer::from_path("gtfs/calendar.txt")?;
    w.serialize(Calendar {
        service_id: "Weekday".to_string(),
        monday: 1, tuesday: 1, wednesday: 1, thursday: 1, friday: 1, saturday: 0, sunday: 0,
        start_date: "20250101".to_string(), end_date: "20261231".to_string(),
    })?;
    w.serialize(Calendar {
        service_id: "Weekend".to_string(),
        monday: 0, tuesday: 0, wednesday: 0, thursday: 0, friday: 0, saturday: 1, sunday: 1,
        start_date: "20250101".to_string(), end_date: "20261231".to_string(),
    })?;

    // Calendar Dates (Holidays)
    let mut w = Writer::from_path("gtfs/calendar_dates.txt")?;
    let holidays = vec![
        "20260101", // New Year's Day
        "20260525", // Memorial Day
        "20260704", // Independence Day
        "20261126", // Thanksgiving
        "20261225", // Christmas
    ];

    for date in holidays {
        w.serialize(CalendarDate {
            service_id: "Weekday".to_string(),
            date: date.to_string(),
            exception_type: 2, // Removed
        })?;
    }

    // Routes - Copy from input or hardcode
    let mut w = Writer::from_path("gtfs/routes.txt")?;
    w.serialize(Route {
        route_id: "5956".to_string(),
        agency_id: "IC".to_string(),
        route_short_name: "IC".to_string(),
        route_long_name: "Irvine Connect".to_string(),
        route_type: 3,
        route_color: "00ABD6".to_string(),
        route_text_color: "FFFFFF".to_string(),
    })?;

    // Shapes - Just copy provided shapes
    fs::copy("input/shapes.txt", "gtfs/shapes.txt")?;

    // Stops - Just copy provided stops
    let mut w = Writer::from_path("gtfs/stops.txt")?;
    for stop in &stops {
        w.serialize(StopOutput {
            stop_id: stop.stop_id.clone(),
            stop_code: "".to_string(),
            stop_name: stop.stop_name.clone(),
            stop_lat: stop.stop_lat,
            stop_lon: stop.stop_lon,
        })?;
    }

    // Trips & Stop Times
    let mut trips_w = Writer::from_path("gtfs/trips.txt")?;
    let mut st_w = Writer::from_path("gtfs/stop_times.txt")?;

    let trip_inputs = data::get_trips();

    for trip_input in trip_inputs {
        trips_w.serialize(Trip {
            route_id: "5956".to_string(),
            service_id: trip_input.service_id.to_string(),
            trip_id: trip_input.trip_id.clone(),
            shape_id: "63618".to_string(),
            block_id: trip_input.block_id.to_string(),
        })?;

        // Construct the sequence of stops for this trip using the file input
        // Since the file might contain multiple loops (172 lines vs 86 stops), 
        // we can use the whole sequence and find the subsequence that matches the timepoints.
        
        let mut trip_stops_sequence: Vec<&RawStop> = Vec::new();
        for id in &stop_sequence_ids {
            if let Some(stop) = stop_map.get(id) {
                trip_stops_sequence.push(stop);
            } else {
                 println!("Warning: Stop ID {} in sequence file not found in stops.txt", id);
            }
        }


        // Now we assign times.
        // Collect all timepoints for this trip.
        // Map them to the indices in `trip_stops_sequence`.
        
        // Strategy: Iterate through `trip_stops_sequence`.
        // Maintain a pointer to current `trip_input.stops` (timepoints).
        // If current stop matches current timepoint, assign time and advance pointer.
        // (Be careful with Duplicate Stop IDs like Dock 4 appearing at start and end).

        let mut final_stop_times: Vec<(u32, String, String, u8)> = Vec::new(); // seq, arr, dep, is_timepoint

        let mut tp_idx = 0;
        
        // Optimization: Pre-calculate indices of timepoints in the Sequence to avoid greedy mismatch
        // (e.g. if Dock 4 appears twice, map 1st timepoint to 1st occurrence, last to last).
        
        // Map each timepoint in `trip_input.stops` to an index in `trip_stops_sequence`.
        let mut key_indices = Vec::new();
        let mut last_search_idx = 0;
        
        for (tp_id_u32, _) in &trip_input.stops {
             let tp_id = tp_id_u32.to_string();
             // Search for this stop in trip_stops_sequence starting from last_search_idx
             if let Some(pos) = trip_stops_sequence.iter().skip(last_search_idx).position(|s| s.stop_id == tp_id) {
                 let absolute_pos = last_search_idx + pos;
                 key_indices.push(absolute_pos);
                 last_search_idx = absolute_pos + 1; // Ensure strict ordering
             } else {
                 println!("Error: Timepoint {} not found in projected sequence after index {}", tp_id, last_search_idx);
                 key_indices.push(last_search_idx); // Fallback to avoid crash, but bad data
             }
        }
        
        // Now interpolate.
        // For segments between timepoints.
        for i in 0..key_indices.len()-1 {
            let start_idx = key_indices[i];
            let end_idx = key_indices[i+1];
            
            let start_time_str = trip_input.stops[i].1.clone().unwrap();
            let end_time_str = trip_input.stops[i+1].1.clone().unwrap();
            
            let start_time = NaiveTime::parse_from_str(&start_time_str, "%H:%M:%S").unwrap();
            let end_time = NaiveTime::parse_from_str(&end_time_str, "%H:%M:%S").unwrap();
            
            let duration = end_time - start_time;
            let num_segments = (end_idx - start_idx) as i64;
            
            for j in 0..=num_segments {
                let current_idx = start_idx + j as usize;
                
                // If we are at the very last point of the whole trip, handle it later or now?
                // The loop handles start to end-1. The last point of segment is start of next.
                // We add stop times for start..end (exclusive of end? No, inclusive?)
                // Standard approach: Add start of segment. Intermediate. 
                // Don't add end of segment (it will be start of next), UNLESS it's the last segment.
                
                if current_idx == end_idx && i < key_indices.len() - 2 {
                    continue; // Skip end of segment, let next segment handle it
                }
                
                // Calculate time
                let added_mins = if num_segments > 0 {
                    duration.num_minutes() * j / num_segments
                } else {
                    0
                };
                let current_time = start_time + chrono::Duration::minutes(added_mins);
                let time_s = current_time.format("%H:%M:%S").to_string();
                
                let stop_id = trip_stops_sequence[current_idx].stop_id.clone();
                let is_tp = if current_idx == start_idx || current_idx == end_idx { 1 } else { 0 };
                
                // Determine headsign
                let forced_headsign = if let Some((last_stop_id_int, _)) = trip_input.stops.last() {
                     if last_stop_id_int.to_string() == "157625" {
                         Some("Yale Ave @ Irvine Blvd")
                     } else {
                         None
                     }
                } else {
                    None
                };

                let headsign = if let Some(h) = forced_headsign {
                    h.to_string()
                } else {
                    // Sequence of 86 stops. Split at index 45 (stop 198259).
                    let idx_in_loop = current_idx % 86;
                    if idx_in_loop < 45 {
                        "Northwood High School".to_string()
                    } else {
                        "Irvine Station".to_string()
                    }
                };

                st_w.serialize(StopTime {
                    trip_id: trip_input.trip_id.clone(),
                    arrival_time: time_s.clone(),
                    departure_time: time_s,
                    stop_id: stop_id,
                    stop_sequence: (current_idx + 1) as u32,
                    stop_headsign: headsign,
                    timepoint: is_tp,
                })?;
            }
        }
        
    }

    Ok(())
}
