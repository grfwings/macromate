//! Simple text format storage for macros
//!
//! Events are stored as raw keycodes (not characters), making playback
//! layout-independent but hardware-specific.

use crate::recorder::RecordedEvent;
use evdev::InputEvent;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

/// Save recorded events to a simple text file
///
/// Format: Each line is: timestamp_us event_type event_code event_value
/// Example: 1234567 1 30 1
pub fn save<P: AsRef<Path>>(path: P, events: &[RecordedEvent]) -> io::Result<()> {
    let mut file = File::create(path)?;

    writeln!(file, "# MacroMate recorded macro")?;
    writeln!(file, "# Format: timestamp_us event_type event_code event_value")?;
    writeln!(file, "# Total events: {}", events.len())?;
    writeln!(file)?;

    for recorded in events {
        writeln!(
            file,
            "{} {} {} {}",
            recorded.timestamp_us,
            recorded.event.event_type().0,
            recorded.event.code(),
            recorded.event.value()
        )?;
    }

    Ok(())
}

/// Load recorded events from a text file
pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Vec<RecordedEvent>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut events = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid line format: {}", line),
            ));
        }

        let timestamp_us: u64 = parts[0].parse().map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Invalid timestamp: {}", e))
        })?;

        let event_type: u16 = parts[1].parse().map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Invalid event_type: {}", e))
        })?;

        let code: u16 = parts[2].parse().map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Invalid code: {}", e))
        })?;

        let value: i32 = parts[3].parse().map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Invalid value: {}", e))
        })?;

        let event = InputEvent::new_now(event_type, code, value);

        events.push(RecordedEvent {
            timestamp_us,
            event,
        });
    }

    Ok(events)
}
