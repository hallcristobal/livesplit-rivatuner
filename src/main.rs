extern crate livesplit_core;
extern crate kernel32;
extern crate winapi;

mod rivatuner;

use livesplit_core::parser::composite;
use livesplit_core::{Timer, HotkeyTimer, Color};
use livesplit_core::component::{timer, splits, title, previous_segment, sum_of_best,
                                possible_time_save};
use std::io::BufReader;
use std::fs::File;
use std::thread;
use std::time::Duration;
use std::fmt::Write;

#[derive(Default)]
struct Components {
    timer: timer::Component,
    splits: splits::Component,
    title: title::Component,
    previous_segment: previous_segment::Component,
    sum_of_best: sum_of_best::Component,
    possible_time_save: possible_time_save::Component,
}

fn format_info_text(text: &str, value: &str) -> String {
    let text = format!("{:<35}", text);
    let value = format!("{:>35}", value);
    text.chars()
        .zip(value.chars())
        .map(|(t, v)| if v.is_whitespace() { t } else { v })
        .collect()
}

fn format_info_text_with_color(text: &str, value: &str, color: Color) -> String {
    let mut result =
        text.chars().take(35usize.saturating_sub(value.chars().count())).collect::<String>();
    let len = result.chars().count();
    result.push_str(map_color(color));
    for _ in 0..35usize.saturating_sub(len).saturating_sub(value.chars().count()) {
        result.push(' ');
    }
    result.push_str(value);
    result.push_str("<C>");
    result
}

fn map_color(color: Color) -> &'static str {
    use livesplit_core::Color::*;
    match color {
        AheadGainingTime => "<C0>",
        AheadLosingTime => "<C1>",
        BehindGainingTime => "<C2>",
        BehindLosingTime => "<C3>",
        BestSegment => "<C4>",
        NotRunning => "<C5>",
        Paused => "<C6>",
        PersonalBest => "<C7>",
        Default => "",
    }
}

fn main() {
    let file = BufReader::new(File::open(r"splits.lss").expect("Can't find splits.lss"));
    let run = composite::parse(file, None, true).expect("Can't parse splits file");
    let timer = Timer::new(run);
    let timer = HotkeyTimer::new(timer).expect("Failed to create global hotkeys");

    let mut components = Components::default();

    let mut text = String::new();

    loop {
        let timer = timer.read();

        text.clear();

        write!(text, "<C0=00CC4B><C1=5CD689><C2=D65C5C><C3=CC0000><C4=FFD500><C5=999999><C6=666666><C7=4DA6FF><C8=4DA6FF><S0=80><S0>").unwrap();

        let state = components.title.state(&timer);

        let category = format!("{:^35}", state.category);
        let attempts = format!("{:>35}", state.attempts);
        let category: String = category.chars()
            .zip(attempts.chars())
            .map(|(c, a)| if a.is_whitespace() { c } else { a })
            .collect();

        writeln!(text,
                 "{:^35}\n{}\n",
                 state.game.chars().take(35).collect::<String>(),
                 category)
            .unwrap();

        let state = components.splits.state(&timer);
        for split in state.splits {
            writeln!(text,
                     "{}{:<15}<C> {}{:>9}<C> {:>9}",
                     if split.is_current_split { "<C8>" } else { "" },
                     split.name.chars().take(15).collect::<String>(),
                     map_color(split.color),
                     split.delta,
                     split.time)
                .unwrap();
        }

        let state = components.timer.state(&timer);
        writeln!(text,
                 "\n{}{:>32}{}<C>\n",
                 map_color(state.color),
                 state.time,
                 state.fraction)
            .unwrap();

        let state = components.previous_segment.state(&timer);
        writeln!(text,
                 "{}",
                 format_info_text_with_color(&state.text, &state.time, state.color))
            .unwrap();

        let state = components.sum_of_best.state(&timer);
        writeln!(text, "{}", format_info_text(&state.text, &state.time)).unwrap();

        let state = components.possible_time_save.state(&timer);
        writeln!(text, "{}<S>", format_info_text(&state.text, &state.time)).unwrap();

        drop(timer);

        text = text.replace('—', "-").replace('−', "-");

        rivatuner::print(&text);

        thread::sleep(Duration::from_millis(33));
    }
}
