use crate::secret::calculate_entropy;
use crate::types::EnvEntry;

pub struct App {
    pub all_entries: Vec<EnvEntry>,
    pub filtered_indices: Vec<usize>,
    pub selected_index: usize,
    pub search_query: String,
    pub is_searching: bool,
    pub global_reveal: bool,
    pub status_message: Option<String>,
}

impl App {
    pub fn new(entries: Vec<EnvEntry>) -> Self {
        let indices: Vec<usize> = (0..entries.len()).collect();
        Self {
            all_entries: entries,
            filtered_indices: indices,
            selected_index: 0,
            search_query: String::new(),
            is_searching: false,
            global_reveal: false,
            status_message: Some("Press '?' or '/' for search, Space to toggle mask, 'q' to quit".to_string()),
        }
    }

    pub fn apply_filter(&mut self) {
        if self.search_query.trim().is_empty() {
            self.filtered_indices = (0..self.all_entries.len()).collect();
        } else {
            let q = self.search_query.to_lowercase();
            self.filtered_indices = self
                .all_entries
                .iter()
                .enumerate()
                .filter(|(_, e)| {
                    e.key.to_lowercase().contains(&q)
                        || e.raw_value.to_lowercase().contains(&q)
                        || e.comment
                            .as_ref()
                            .map(|c| c.to_lowercase().contains(&q))
                            .unwrap_or(false)
                })
                .map(|(i, _)| i)
                .collect();
        }

        if self.selected_index >= self.filtered_indices.len() && !self.filtered_indices.is_empty() {
            self.selected_index = self.filtered_indices.len() - 1;
        }
    }

    pub fn selected_entry(&self) -> Option<&EnvEntry> {
        if self.filtered_indices.is_empty() {
            None
        } else {
            let idx = self.filtered_indices[self.selected_index];
            Some(&self.all_entries[idx])
        }
    }

    pub fn next(&mut self) {
        if !self.filtered_indices.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.filtered_indices.len();
        }
    }

    pub fn previous(&mut self) {
        if !self.filtered_indices.is_empty() {
            if self.selected_index == 0 {
                self.selected_index = self.filtered_indices.len() - 1;
            } else {
                self.selected_index -= 1;
            }
        }
    }

    pub fn toggle_global_reveal(&mut self) {
        self.global_reveal = !self.global_reveal;
        self.status_message = Some(if self.global_reveal {
            "Secrets REVEALED!".to_string()
        } else {
            "Secrets MASKED.".to_string()
        });
    }

    pub fn get_entropy_score(&self) -> f64 {
        if let Some(entry) = self.selected_entry() {
            calculate_entropy(&entry.raw_value)
        } else {
            0.0
        }
    }
}
