use indicatif::{ProgressBar, ProgressStyle};

const PROGRESS_BAR_LENGTH: u64 = 100;

pub fn get_progress_bar(text: &str) -> ProgressBar {
    let progress_bar = ProgressBar::new(PROGRESS_BAR_LENGTH);
    let style = ProgressStyle::default_bar()
        .template("{prefix} [{bar:40.cyan/blue}] {percent}% {eta} {msg}")
        .unwrap()
        .progress_chars("=> ");
    progress_bar.set_style(style);
    progress_bar.set_prefix(text.to_string());

    progress_bar
}
