//! Utility functions.
//!
//! This module contains utility functions that are used in multiple places in the application.

use gtk::prelude::EntryBufferExtManual;
use gtk::traits::EntryExt;
use gtk::Entry;

/// Get the text from an entry and clear it.
///
/// Returns `None` if the entry is empty.
pub fn get_and_clear_entry(entry: Entry) -> Option<String> {
    let buffer = entry.buffer();
    let message = buffer.text().to_string();

    if message.is_empty() {
        return None;
    }

    buffer.set_text("");
    Some(message)
}
