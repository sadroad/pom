use indicatif::{ProgressBar, ProgressStyle};
use inquire::Select;
use notify_rust::Notification;
use std::{io::Cursor, process::exit, thread::sleep, time::Duration};

fn main() {
    let work_complete = Notification::new()
        .summary("Pomodoro")
        .body("Work Done")
        .finalize();
    let break_done = Notification::new()
        .summary("Pomodoro")
        .body("Break Over")
        .finalize();

    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
        .expect("Default audio stream unable to open");

    let alert_file = include_bytes!("alert.mp3");
    let sink = rodio::Sink::connect_new(stream_handle.mixer());
    loop {
        let options = vec!["25m/5m", "50m/10m", "all done"];
        let time_choice = Select::new("Split", options).prompt();
        let time = match time_choice {
            Ok(choice) => match choice {
                "25m/5m" => (25 * 60, 5 * 60),
                "50m/10m" => (50 * 60, 10 * 60),
                _ => exit(0),
            },
            Err(_) => exit(1),
        };
        let pb = ProgressBar::new(time.0).with_style(
            ProgressStyle::with_template("{bar:40.cyan/blue} [{elapsed_precise}]")
                .unwrap()
                .progress_chars("##-"),
        );
        for _ in 0..time.0 {
            pb.inc(1);
            sleep(Duration::from_secs(1));
        }
        pb.finish_and_clear();
        _ = work_complete.show();
        sink.append(rodio::Decoder::new(Cursor::new(alert_file)).unwrap());
        sink.sleep_until_end();

        let options = vec!["Yes", "No"];
        let confirm = Select::new("Ready for a break?", options).prompt();
        let take_break = match confirm {
            Ok(choice) => match choice {
                "Yes" => true,
                "No" => false,
                _ => exit(1),
            },
            Err(_) => exit(1),
        };

        if !take_break {
            continue;
        }

        let pb = ProgressBar::new(time.1).with_style(
            ProgressStyle::with_template("{bar:40.cyan/blue} [{elapsed_precise}]")
                .unwrap()
                .progress_chars("##-"),
        );
        for _ in 0..time.1 {
            pb.inc(1);
            sleep(Duration::from_secs(1));
        }
        pb.finish_and_clear();
        _ = break_done.show();
        sink.append(rodio::Decoder::new(Cursor::new(alert_file)).unwrap());
        sink.sleep_until_end();
    }
}
