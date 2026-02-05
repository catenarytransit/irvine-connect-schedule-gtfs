use chrono::NaiveTime;

pub struct TripInput {
    pub trip_id: String,
    pub service_id: &'static str,
    pub block_id: &'static str,
    pub stops: Vec<(u32, Option<String>)>,
}

#[derive(Clone, Copy)]
pub enum Pattern {
    Full,       // To Sand Canyon, then Return to Dock 4 (0-7)
    ShortYale,  // To Yale/Irvine (0-4)
    StartYale,  // From Yale/Irvine to Dock 4 (Morning setup) (4-7)
}

// Timepoints
// 0: Dock 4 (157583)
// 1: Alton/Hoag (157593)
// 2: Lake/Barranca (157601)
// 3: Yale/Bryan (198349)
// 4: Yale/Irvine (157625)
// 5: Yale/Deerfield (157667)
// 6: Sand Canyon/Hoag (157641)
// 7: Dock 4 (Return) (157583)

const TIMEPOINTS: [u32; 7] = [
    157583, // Dock 4
    157593, // Alton/Hoag
    157601, // Lake/Barranca
    198349, // Yale/Bryan
    157625, // Yale/Irvine
    157667, // Yale/Deerfield
    157641, // Sand Canyon/Hoag
];

const OFFSETS: [i64; 8] = [
    0,   // Dock 4
    15,  // Alton/Hoag
    30,  // Lake/Barranca
    45,  // Yale/Bryan
    65,  // Yale/Irvine
    75,  // Yale/Deerfield
    95,  // Sand Canyon/Hoag
    110, // Dock 4 (Return - Estimated 15 mins from Sand Canyon)
];

struct RawTrip {
    bus_id: u32,
    block_id: &'static str,
    start_time: &'static str,
    pattern: Pattern,
}

pub fn get_trips() -> Vec<TripInput> {
    let mut trips = Vec::new();

    // WEEKDAY (Mon-Fri)
    let service_mf = "Weekday";
    let trips_mf = vec![
        // Bus 1 (0520)
        RawTrip { bus_id: 1, block_id: "0520", start_time: "06:00", pattern: Pattern::Full },
        RawTrip { bus_id: 1, block_id: "0520", start_time: "08:00", pattern: Pattern::Full },
        RawTrip { bus_id: 1, block_id: "0520", start_time: "09:50", pattern: Pattern::Full },
        RawTrip { bus_id: 1, block_id: "0520", start_time: "11:50", pattern: Pattern::Full },
        RawTrip { bus_id: 1, block_id: "0520", start_time: "13:55", pattern: Pattern::Full },
        RawTrip { bus_id: 1, block_id: "0520", start_time: "15:50", pattern: Pattern::Full },
        RawTrip { bus_id: 1, block_id: "0520", start_time: "17:55", pattern: Pattern::Full },
        
        // Bus 2 (0535)
        RawTrip { bus_id: 2, block_id: "0535", start_time: "06:20", pattern: Pattern::Full },
        RawTrip { bus_id: 2, block_id: "0535", start_time: "08:20", pattern: Pattern::Full },
        RawTrip { bus_id: 2, block_id: "0535", start_time: "10:10", pattern: Pattern::Full },
        RawTrip { bus_id: 2, block_id: "0535", start_time: "12:10", pattern: Pattern::Full },
        RawTrip { bus_id: 2, block_id: "0535", start_time: "14:15", pattern: Pattern::Full },
        RawTrip { bus_id: 2, block_id: "0535", start_time: "16:10", pattern: Pattern::Full },
        RawTrip { bus_id: 2, block_id: "0535", start_time: "18:15", pattern: Pattern::Full },

        // Bus 3 (0600)
        RawTrip { bus_id: 3, block_id: "0600", start_time: "06:40", pattern: Pattern::Full },
        RawTrip { bus_id: 3, block_id: "0600", start_time: "08:40", pattern: Pattern::Full },
        RawTrip { bus_id: 3, block_id: "0600", start_time: "10:30", pattern: Pattern::Full },
        RawTrip { bus_id: 3, block_id: "0600", start_time: "12:30", pattern: Pattern::Full },
        RawTrip { bus_id: 3, block_id: "0600", start_time: "14:35", pattern: Pattern::Full },
        RawTrip { bus_id: 3, block_id: "0600", start_time: "16:30", pattern: Pattern::Full },
        RawTrip { bus_id: 3, block_id: "0600", start_time: "18:35", pattern: Pattern::ShortYale }, // Ends 19:40

        // Bus 4 (0520)
        RawTrip { bus_id: 4, block_id: "0520", start_time: "06:00", pattern: Pattern::StartYale },
        RawTrip { bus_id: 4, block_id: "0520", start_time: "07:00", pattern: Pattern::Full },
        RawTrip { bus_id: 4, block_id: "0520", start_time: "08:55", pattern: Pattern::Full },
        RawTrip { bus_id: 4, block_id: "0520", start_time: "10:50", pattern: Pattern::Full },
        RawTrip { bus_id: 4, block_id: "0520", start_time: "12:50", pattern: Pattern::Full },
        RawTrip { bus_id: 4, block_id: "0520", start_time: "14:55", pattern: Pattern::Full },
        RawTrip { bus_id: 4, block_id: "0520", start_time: "16:50", pattern: Pattern::Full },
        RawTrip { bus_id: 4, block_id: "0520", start_time: "18:55", pattern: Pattern::ShortYale }, // Ends 20:00

        // Bus 5 (0535)
        RawTrip { bus_id: 5, block_id: "0535", start_time: "06:20", pattern: Pattern::StartYale },
        RawTrip { bus_id: 5, block_id: "0535", start_time: "07:20", pattern: Pattern::Full },
        RawTrip { bus_id: 5, block_id: "0535", start_time: "09:15", pattern: Pattern::Full },
        RawTrip { bus_id: 5, block_id: "0535", start_time: "11:20", pattern: Pattern::Full },
        RawTrip { bus_id: 5, block_id: "0535", start_time: "13:10", pattern: Pattern::Full },
        RawTrip { bus_id: 5, block_id: "0535", start_time: "15:10", pattern: Pattern::Full },
        RawTrip { bus_id: 5, block_id: "0535", start_time: "17:10", pattern: Pattern::Full },
        RawTrip { bus_id: 5, block_id: "0535", start_time: "19:15", pattern: Pattern::ShortYale }, // Ends 20:20

        // Bus 6 (0550)
        RawTrip { bus_id: 6, block_id: "0550", start_time: "06:40", pattern: Pattern::StartYale },
        RawTrip { bus_id: 6, block_id: "0550", start_time: "07:40", pattern: Pattern::Full },
        RawTrip { bus_id: 6, block_id: "0550", start_time: "09:35", pattern: Pattern::Full },
        RawTrip { bus_id: 6, block_id: "0550", start_time: "11:40", pattern: Pattern::Full },
        RawTrip { bus_id: 6, block_id: "0550", start_time: "13:30", pattern: Pattern::Full },
        RawTrip { bus_id: 6, block_id: "0550", start_time: "15:30", pattern: Pattern::Full },
        RawTrip { bus_id: 6, block_id: "0550", start_time: "17:30", pattern: Pattern::Full },
        // Bus 6 last trip inferred
        RawTrip { bus_id: 6, block_id: "0550", start_time: "19:35", pattern: Pattern::ShortYale }, // Ends 20:40
    ];

    process_trips(&mut trips, trips_mf, service_mf);

    // WEEKEND (Sat-Sun)
    let service_we = "Weekend";
    let trips_we = vec![
        // Bus 1 (0720)
        RawTrip { bus_id: 1, block_id: "0720", start_time: "08:00", pattern: Pattern::Full },
        RawTrip { bus_id: 1, block_id: "0720", start_time: "10:00", pattern: Pattern::Full },
        RawTrip { bus_id: 1, block_id: "0720", start_time: "11:50", pattern: Pattern::Full },
        RawTrip { bus_id: 1, block_id: "0720", start_time: "13:50", pattern: Pattern::Full },
        RawTrip { bus_id: 1, block_id: "0720", start_time: "15:55", pattern: Pattern::Full },
        RawTrip { bus_id: 1, block_id: "0720", start_time: "17:50", pattern: Pattern::Full },
        RawTrip { bus_id: 1, block_id: "0720", start_time: "19:55", pattern: Pattern::Full },

        // Bus 2 (0735)
        RawTrip { bus_id: 2, block_id: "0735", start_time: "08:20", pattern: Pattern::Full },
        RawTrip { bus_id: 2, block_id: "0735", start_time: "10:20", pattern: Pattern::Full },
        RawTrip { bus_id: 2, block_id: "0735", start_time: "12:10", pattern: Pattern::Full },
        RawTrip { bus_id: 2, block_id: "0735", start_time: "14:10", pattern: Pattern::Full },
        RawTrip { bus_id: 2, block_id: "0735", start_time: "16:15", pattern: Pattern::Full },
        RawTrip { bus_id: 2, block_id: "0735", start_time: "18:15", pattern: Pattern::Full },
        RawTrip { bus_id: 2, block_id: "0735", start_time: "20:15", pattern: Pattern::Full },

        // Bus 3 (0800)
        RawTrip { bus_id: 3, block_id: "0800", start_time: "08:40", pattern: Pattern::Full },
        RawTrip { bus_id: 3, block_id: "0800", start_time: "10:40", pattern: Pattern::Full },
        RawTrip { bus_id: 3, block_id: "0800", start_time: "12:30", pattern: Pattern::Full },
        RawTrip { bus_id: 3, block_id: "0800", start_time: "14:30", pattern: Pattern::Full },
        RawTrip { bus_id: 3, block_id: "0800", start_time: "16:30", pattern: Pattern::Full },
        RawTrip { bus_id: 3, block_id: "0800", start_time: "18:30", pattern: Pattern::Full },
        RawTrip { bus_id: 3, block_id: "0800", start_time: "20:35", pattern: Pattern::ShortYale }, // Ends 21:40

        // Bus 4 (0720)
        RawTrip { bus_id: 4, block_id: "0720", start_time: "09:00", pattern: Pattern::Full },
        RawTrip { bus_id: 4, block_id: "0720", start_time: "10:55", pattern: Pattern::Full },
        RawTrip { bus_id: 4, block_id: "0720", start_time: "12:50", pattern: Pattern::Full },
        RawTrip { bus_id: 4, block_id: "0720", start_time: "14:50", pattern: Pattern::Full },
        RawTrip { bus_id: 4, block_id: "0720", start_time: "16:55", pattern: Pattern::Full },
        RawTrip { bus_id: 4, block_id: "0720", start_time: "18:50", pattern: Pattern::Full },
        RawTrip { bus_id: 4, block_id: "0720", start_time: "20:55", pattern: Pattern::ShortYale }, // Ends 22:00

        // Bus 5 (0735)
        RawTrip { bus_id: 5, block_id: "0735", start_time: "09:20", pattern: Pattern::Full },
        RawTrip { bus_id: 5, block_id: "0735", start_time: "11:15", pattern: Pattern::Full },
        RawTrip { bus_id: 5, block_id: "0735", start_time: "13:20", pattern: Pattern::Full },
        RawTrip { bus_id: 5, block_id: "0735", start_time: "15:15", pattern: Pattern::Full },
        RawTrip { bus_id: 5, block_id: "0735", start_time: "17:15", pattern: Pattern::Full },
        RawTrip { bus_id: 5, block_id: "0735", start_time: "19:10", pattern: Pattern::Full },
        RawTrip { bus_id: 5, block_id: "0735", start_time: "21:15", pattern: Pattern::ShortYale }, // Ends 22:20

        // Bus 6 (0750)
        RawTrip { bus_id: 6, block_id: "0750", start_time: "09:40", pattern: Pattern::Full },
        RawTrip { bus_id: 6, block_id: "0750", start_time: "11:35", pattern: Pattern::Full },
        RawTrip { bus_id: 6, block_id: "0750", start_time: "13:40", pattern: Pattern::Full },
        RawTrip { bus_id: 6, block_id: "0750", start_time: "15:40", pattern: Pattern::Full },
        RawTrip { bus_id: 6, block_id: "0750", start_time: "17:35", pattern: Pattern::Full },
        RawTrip { bus_id: 6, block_id: "0750", start_time: "19:35", pattern: Pattern::Full }, // Added inferred Full
        RawTrip { bus_id: 6, block_id: "0750", start_time: "21:35", pattern: Pattern::ShortYale }, // Added inferred Short
    ];
    
    process_trips(&mut trips, trips_we, service_we);

    trips
}

fn process_trips(trips: &mut Vec<TripInput>, raw_trips: Vec<RawTrip>, service_id: &'static str) {
    for (i, trip) in raw_trips.into_iter().enumerate() {
        let start = NaiveTime::parse_from_str(trip.start_time, "%H:%M").unwrap();
        
        // Define range based on pattern
        let (min_idx, max_idx) = match trip.pattern {
            Pattern::Full => (0, 7),
            Pattern::ShortYale => (0, 4),
            Pattern::StartYale => (4, 7),
        };

        let mut stops = Vec::new();

        // Calculate offset adjustment. 
        // trip.start_time is assumed to be the time at min_idx.
        // So time at idx = start + (OFFSETS[idx] - OFFSETS[min_idx])
        let base_offset = OFFSETS[min_idx];

        for (idx, offset) in OFFSETS.iter().enumerate() {
            if idx < min_idx || idx > max_idx { continue; }
            
            // Calculate relative offset from the start of this specific trip
            let relative_offset = *offset - base_offset;
            
            let time = start + chrono::Duration::minutes(relative_offset);
            let time_str = time.format("%H:%M:%S").to_string();
            
            // For Index 7 (Return Dock 4), the ID is SAME as Index 0
            let stop_id = if idx == 7 { TIMEPOINTS[0] } else { TIMEPOINTS[idx] };
            
            stops.push((stop_id, Some(time_str)));
        }
        
        trips.push(TripInput {
            trip_id: format!("{}_{}_{}", service_id.to_lowercase(), trip.bus_id, i + 1),
            service_id: service_id,
            block_id: trip.block_id,
            stops: stops.into_iter().map(|(id, t)| (id, t)).collect(),
        });
    }
}
