# Irvine Connect GTFS Generator

This tool generates a GTFS feed for the Irvine Connect bus based on geospatial files from Passio GTFS and schedule data obtained through freedom of information act requests.

## How to Run

1.  Ensure you have Rust installed.
2.  Run the generator:
    ```bash
    cargo run
    ```
3.  The GTFS files will be generated in the `gtfs/` directory.
4.  Package the feed into a ZIP file:
    ```bash
    cd gtfs && zip -r ../gtfs.zip * && cd ..
    ```
    This creates `gtfs.zip` in the root directory.

## How the Table Works (`src/data.rs`)

The schedule data is defined in `src/data.rs`. Instead of listing every single stop time, the system uses a **Pattern** and **Offset** based approach to keep the data concise and easy to update.

### Core Concepts

*   **Timepoints**: A list of key stops (stops with specific scheduled times). Defined in `TIMEPOINTS` array.
*   **Offsets**: The number of minutes it takes to reach each timepoint from the start of the trip. Defined in `OFFSETS` array.
    *   *Example*: If `Offsets` is `[0, 15, 30...]`, a trip starting at 8:00 AM will be at the second timepoint at 8:15 AM.
*   **Patterns**:
    *   `Pattern::Full`: The bus runs the full loop from Dock 4 to Sand Canyon and returns to Dock 4.
    *   `Pattern::ShortYale`: The bus terminates early at Yale/Irvine (used for end-of-day trips).

### Updating the Schedule

To add or modify trips, edit `src/data.rs`.

Locate the `get_trips()` function. You will see vectors for Weekday (`trips_mf`) and Weekend (`trips_we`).

Add a new trip entry like this:

```rust
RawTrip { 
    bus_id: 1,          // The bus number (internal logic)
    block_id: "0520",   // The block number from the schedule header
    start_time: "06:00",// Time in HH:MM (24-hour format)
    pattern: Pattern::Full 
},
```

### Updating Holidays

Holiday exceptions (dates with NO service) are hardcoded in `src/main.rs` under the `// Calendar Dates (Holidays)` section. Add strings in `YYYYMMDD` format to the `holidays` vector.

## Input Files

*   `input/stops.txt`: Static list of stops.
*   `input/shapes.txt`: The route geometry.
*   `input/stop_id_sequence.txt`: The ordered list of Stop IDs the bus visits on its route. This handles the loop logic.
