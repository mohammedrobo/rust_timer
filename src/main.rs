use std::time::{Duration, Instant};
use std::thread;
use std::env;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use notify_rust::Notification;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use rand::seq::SliceRandom;

/// A simple, minimalist terminal countdown timer with desktop notifications.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Set the timer duration in hours
    #[arg(short = 'H', long, default_value_t = 0)]
    hours: u64,

    /// Set the timer duration in minutes
    #[arg(short = 'm', long, default_value_t = 0)]
    minutes: u64,

    /// Set the timer duration in seconds
    #[arg(short = 's', long, default_value_t = 0)]
    seconds: u64,

    /// Optional message to display in the notification
    #[arg(short = 'M', long, default_value = "Your timer is complete!")]
    message: String,
}

fn main() {
    let args = Args::parse();

    let total_seconds = (args.hours * 3600) + (args.minutes * 60) + args.seconds;
    let total_duration = Duration::from_secs(total_seconds);

    if total_duration.is_zero() {
        println!("Error: No duration specified. Use -h, -m, or -s to set a time.");
        println!("For help, run with --help");
        return;
    }

    let start_time = Instant::now();

    // --- UI Setup ---
    let pb = ProgressBar::new(total_duration.as_secs());
    pb.set_style(ProgressStyle::default_bar()
        .template("⏳ {msg} [{bar:40.cyan/blue}] [{elapsed}]")
        .expect("Failed to create progress bar style")
        .progress_chars("#-"));

    println!("Starting timer for {}...", humantime::format_duration(total_duration));

    // --- Timer Logic ---
    while Instant::now() - start_time < total_duration {
        let elapsed = Instant::now() - start_time;
        let remaining = total_duration.saturating_sub(elapsed);

        let elapsed_formatted = format!(
            "{:02}:{:02}:{:02}",
            elapsed.as_secs() / 3600,
            (elapsed.as_secs() % 3600) / 60,
            elapsed.as_secs() % 60
        );

        let remaining_message = if args.hours > 0 {
            format!("{:.0}h remaining", remaining.as_secs_f64() / 3600.0)
        } else if args.minutes > 0 {
            format!("{:.0}m remaining", remaining.as_secs_f64() / 60.0)
        } else {
            format!("{}s remaining", remaining.as_secs())
        };

        pb.set_position(elapsed.as_secs());
        pb.set_message(remaining_message);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(&format!(
                    "⏳ {{msg}} [{{bar:40.cyan/blue}}] [{}]",
                    elapsed_formatted
                ))
                .expect("Failed to create progress bar style")
                .progress_chars("#-"),
        );

        thread::sleep(Duration::from_millis(250));
    }

    // --- Completion ---
    pb.finish_with_message("✅ Timer finished!");

    // 1. Play Sound
    if let Err(e) = play_sound() {
        // Non-critical error, so we just print it
        eprintln!("🔔 Could not play sound: {}", e);
    }

    // 2. Send Desktop Notification
    Notification::new()
        .summary("⏰ Timer Finished!")
        .body(&args.message)
        .icon("accessories-clock") // Use a standard system icon as a fallback
        .show()
        .expect("Failed to send notification");

    // 3. Print Final Message with Icon
    let motivational_messages = [
        "🚀 Great work!",
        "🎉 You did it!",
        "🏆 Success!",
        "✨ Well done!",
        "🎯 On to the next one!",
    ];
    let mut rng = rand::thread_rng();
    let message = motivational_messages.choose(&mut rng).unwrap_or(&"🔔 Time's up!");
    println!("{}", message);
}

/// Plays a sound from a file named "complete" with a common audio extension.
/// It checks for "complete.wav", "complete.mp3", and "complete.ogg" in the
/// same directory as the executable.
fn play_sound() -> Result<(), Box<dyn std::error::Error>> {
    let exe_path = env::current_exe()?;
    let exe_dir = exe_path.parent().ok_or("Failed to get executable directory")?;

    let sound_file_name = ["complete.wav", "complete.mp3", "complete.ogg"]
        .iter()
        .find(|file| exe_dir.join(file).exists())
        .ok_or("No sound file (complete.wav/mp3/ogg) found near executable")?;

    let sound_path = exe_dir.join(sound_file_name);

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    let file = File::open(&sound_path)?;
    let source = Decoder::new(BufReader::new(file))?;
    sink.append(source);

    // Block the thread until the sound has finished playing
    sink.sleep_until_end();
    Ok(())
}
