# rust_timer ⏰

A simple, minimalist terminal countdown timer for Linux with desktop notifications and sound, built with Rust.

![rust_timer_demo](https://i.imgur.com/YOUR_IMAGE_URL.gif) 
*(Optional: You can run the timer and record a GIF of it in action to show it off!)*

---

## ✨ Features

- **Simple Interface:** Set timers from your terminal using hours, minutes, and seconds.
- **Visual Feedback:** A clean, animated progress bar shows the time remaining.
- **Desktop Notifications:** Get a system notification when the timer is complete.
- **Sound Alerts:** Plays a sound to get your attention.
- **Highly Performant:** Built with Rust for minimal resource consumption.
- **System-Wide:** Install it once and run it from any directory.

## 📦 Installation

1.  **Prerequisites:** Ensure you have the Rust toolchain (`cargo`) and `ffmpeg` installed.
2.  **Clone the repository:**
    ```bash
    git clone https://github.com/your-username/rust_timer.git
    cd rust_timer
    ```
3.  **Build the release binary:**
    ```bash
    cargo build --release
    ```
4.  **Install the executable:**
    ```bash
    cp target/release/rust_timer ~/.local/bin/
    ```
    *(Ensure `~/.local/bin` is in your system's PATH)*

5.  **Install Assets:** Place a sound file (`complete.wav`, `.mp3`, or `.ogg`) in `~/.local/bin/`. The notification icon is a standard system icon.

## 🚀 Usage

Run the timer from any terminal.

**Examples:**

- **Set a timer for 10 seconds:**
  ```bash
  rust_timer -s 10
  ```
- **Set a timer for 5 minutes and 30 seconds:**
  ```bash
  rust_timer -m 5 -s 30
  ```
- **Set a timer for 1 hour:**
  ```bash
  rust_timer -H 1
  ```

### Options

| Flag | Long Flag | Description                  | Default     |
| :--- | :-------- | :--------------------------- | :---------- |
| `-H` | `--hours`   | Set the duration in hours    | `0`         |
| `-m` | `--minutes` | Set the duration in minutes  | `0`         |
| `-s` | `--seconds` | Set the duration in seconds  | `0`         |
| `-M` | `--message` | Set a custom notification message | "Your timer is complete!" |

---

## 🔧 Contributing

Feel free to open an issue to report a bug or suggest a feature, or submit a pull request with improvements.
