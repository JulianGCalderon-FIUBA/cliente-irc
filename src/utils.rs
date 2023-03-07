use gtk::{prelude::EntryBufferExtManual, traits::EntryExt, Entry};

/// If entry has content, it returns it, clearing the entry
pub fn get_and_clear_entry(entry: Entry) -> Option<String> {
    let buffer = entry.buffer();
    let message = buffer.text().to_string();

    if message.is_empty() {
        return None;
    }

    buffer.set_text("");
    Some(message)
}
