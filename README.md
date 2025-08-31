#  rust_timer ⏰

A simple, minimalist, and elegant terminal countdown timer for Linux, built with Rust.

It provides a clean visual progress bar, desktop notifications, and sound alerts, making it a perfect tool for productivity sessions, reminders, or any countdown need directly from your terminal.

![rust_timer_demo](https://i.imgur.com/YOUR_IMAGE_URL.gif)
*(Optional: You can run the timer and record a GIF of it in action to show it off!)*

---

## ✨ Features

- **Elegant Interface:** Set timers easily using hours, minutes, and seconds.
- **Clean Visual Feedback:** A smooth, animated progress bar shows the time remaining.
- **Desktop Notifications:** Get a native system notification when the timer is complete.
- **Customizable Sound Alerts:** Plays a sound to get your attention when time is up.
- **Motivational Messages:** Get a random motivational message upon completion.
- **Cross-Platform:** Works on any modern Linux desktop environment (GNOME, KDE, XFCE, etc.).
- **Lightweight & Performant:** Built with Rust for minimal resource consumption.

## 📦 Installation

#### Prerequisites
- **Rust Toolchain:** `rustc` and `cargo`
- **For sound:** `ffmpeg` and `libasound2-dev` (or your system's equivalent ALSA development library).

#### Build from Source
1.  **Clone the repository:**
    ```bash
    git clone https://github.com/mohammedrobo/rust_timer.git
    cd rust_timer
    ```

2.  **Build the release binary:**
    ```bash
    cargo build --release
    ```

3.  **Install the executable:**
    ```bash
    cp target/release/rust_timer ~/.local/bin/
    ```
    *(Ensure `~/.local/bin` is in your system's PATH)*

4.  **(Optional) Install Sound Asset:**
    Place a sound file named `complete.wav`, `complete.mp3`, or `complete.ogg` in the same directory as the executable (`~/.local/bin/`).

## 🚀 Usage

Run the timer from any terminal.

#### Examples

- **Set a timer for 10 seconds:**
  ```bash
  rust_timer -s 10
  ```
- **Set a timer for 5 minutes and 30 seconds:**
  ```bash
  rust_timer -m 5 -s 30
  ```
- **Set a timer for 1 hour with a custom message:**
  ```bash
  rust_timer -H 1 -M "Time to take a break!"
  ```

### Command-Line Options

| Flag | Long Flag   | Description                       | Default                   |
| :--- | :---------- | :-------------------------------- | :------------------------ |
| `-H` | `--hours`   | Set the duration in hours         | `0`                       |
| `-m` | `--minutes` | Set the duration in minutes       | `0`                       |
| `-s` | `--seconds` | Set the duration in seconds       | `0`                       |
| `-M` | `--message` | Set a custom notification message | "Your timer is complete!" |

---

## 🔧 Contributing

Contributions are welcome! Feel free to open an issue to report a bug or suggest a feature, or submit a pull request with your improvements.

1.  Fork the repository.
2.  Create your feature branch (`git checkout -b feature/AmazingFeature`).
3.  Commit your changes (`git commit -m 'Add some AmazingFeature'`).
4.  Push to the branch (`git push origin feature/AmazingFeature`).
5.  Open a Pull Request.

## 📜 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.