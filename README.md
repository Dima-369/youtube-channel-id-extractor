# YouTube Channel ID Extractor

The **YouTube Channel ID Extractor** is a command-line tool that extracts a YouTube Channel ID from a provided URL using YouTube's Data API v3.

## Features

- Parses a YouTube Channel URL to extract the handle.
- Utilizes the YouTube Data API to retrieve the corresponding Channel ID.
- Requires minimal setup and is easy to use.

---

## Requirements

- **YouTube Data API v3** enabled in Google Cloud Console.
- A valid **API key** for authentication with the YouTube API.
- Rust Compiler (to build and run the application).

---

## Setup

1. Go to <https://console.cloud.google.com>.
2. Create a new project → Enable the **"YouTube Data API v3"** for your project.
3. Create credentials → API Key.
4. Set the environment variable `YOUTUBE_API_KEY` to the API Key generated:
   ```bash
   export YOUTUBE_API_KEY=<your_api_key>
   ```

---

## How to Use

1. Clone the repository and navigate to the project directory:
   ```bash
   git clone <repository_url>
   cd youtube-channel-id-extractor
   ```
2. Build the project using Cargo:
   ```bash
   cargo build --release
   ```
3. Run the binary with the **YouTube channel URL** as an argument:
   ```bash
   ./target/release/youtube-channel-id-extractor "<youtube_channel_url>"
   ```

   Example:
   ```bash
   ./target/release/youtube-channel-id-extractor "https://www.youtube.com/@example_channel"
   ```

   This will output the `Channel ID` corresponding to the provided URL.

---

## Example Workflow

1. Enter the **YouTube channel URL.**
2. The tool extracts the handle (e.g., `@example_channel`) from the URL.
3. Calls the YouTube API to locate the channel information.
4. Outputs the Channel ID, e.g., `UC1234567890EXAMPLE`.

---

## Error Handling

- Ensure the provided URL is in a valid YouTube channel format (e.g., `https://www.youtube.com/@someChannel`).
- Make sure the `YOUTUBE_API_KEY` environment variable is set and valid.
- The tool will display an error message if the channel cannot be found in the API response.
