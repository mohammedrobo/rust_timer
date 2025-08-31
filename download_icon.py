import requests

url = "https://static.vecteezy.com/system/resources/previews/024/029/779/original/hourglass-icon-sand-clock-timer-symbol-free-png.png"
try:
    response = requests.get(url, timeout=10)
    response.raise_for_status()  # Raise an exception for bad status codes
    with open("/home/satoru/.local/bin/timer-icon.png", "wb") as f:
        f.write(response.content)
    print("Icon downloaded successfully.")
except requests.exceptions.RequestException as e:
    print(f"Error downloading icon: {e}")
    exit(1)
