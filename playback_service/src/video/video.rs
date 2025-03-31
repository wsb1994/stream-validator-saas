use std::env;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

pub fn check_hls_stream(url: &str) -> Result<bool, Box<dyn std::error::Error>> {
    // Configure FFmpeg command
    // -v error: Only show errors
    // -analyzeduration 10M: Increase analysis time for complex streams
    // -probesize 10M: Increase probe size for better format detection
    // -i: Input file (the HLS URL)
    // -frames:v 5: Only process 5 video frames
    // -an: Disable audio processing
    // -f null -: Output to null device
    let mut ffmpeg_process = Command::new("ffmpeg")
        .args([
            "-v",
            "error",
            "-analyzeduration",
            "10M",
            "-probesize",
            "10M",
            "-i",
            url,
            "-frames:v",
            "5",
            "-an",
            "-f",
            "null",
            "-",
        ])
        .stderr(Stdio::piped())
        .spawn()?;

    // Capture and process stderr output from FFmpeg
    // Use as_mut to borrow the Option<ChildStderr> without moving it
    let stderr = ffmpeg_process
        .stderr
        .take()
        .ok_or("Failed to capture stderr")?;
    let reader = BufReader::new(stderr);

    // Check if there are any critical errors in the output
    let mut has_errors = false;
    for line in reader.lines() {
        let line = line?;
        if line.contains("Invalid data found")
            || line.contains("Server returned 404")
            || line.contains("Connection refused")
            || line.contains("Unable to open resource")
        {
            has_errors = true;
            eprintln!("FFmpeg error: {}", line);
        }
    }

    // Wait for the process to finish and check exit status
    let exit_status = ffmpeg_process.wait()?;

    // Return true if FFmpeg exited successfully and no critical errors were found
    Ok(exit_status.success() && !has_errors)
}
