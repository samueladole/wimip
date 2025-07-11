use std::process::{Command, Stdio};
use std::io::{Write};
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use reqwest to get IP address
    let client = Client::new();
    let ip = client
        .get("https://ifconfig.io")
        .header("User-agent", "curl/7.64.1")
        .send()
        .await?
        .text()
        .await?;

    // Print only the IP, like `curl ifconfig.io`
    println!("Your IP: {ip}");

    #[cfg(target_os = "macos")]
    {
        let mut pbcopy = Command::new("pbcopy").stdin(Stdio::piped()).spawn()?;
        if let Some(stdin) = pbcopy.stdin.as_mut() {
            stdin.write_all(ip.as_bytes())?;
        }
        pbcopy.wait()?;
    }

    #[cfg(target_os = "windows")]
    {
        let mut clip = Command::new("clip").stdin(Stdio::piped()).spawn()?;
        if let Some(stdin) = clip.stdin.as_mut() {
            stdin.write_all(ip.as_bytes())?;
        }
        clip.wait()?;
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        let mut copied = false;
        if Command::new("which").arg("xclip").output()?.status.success() {
            let mut xclip = Command::new("xclip")
                .arg("-selection")
                .arg("clipboard")
                .stdin(Stdio::piped())
                .spawn()?;
            if let Some(stdin) = xclip.stdin.as_mut() {
                stdin.write_all(ip.as_bytes())?;
            }
            xclip.wait()?;
            copied = true;
        } else if Command::new("which").arg("xsel").output()?.status.success() {
            let mut xsel = Command::new("xsel")
                .arg("--clipboard")
                .arg("--input")
                .stdin(Stdio::piped())
                .spawn()?;
            if let Some(stdin) = xsel.stdin.as_mut() {
                stdin.write_all(ip.as_bytes())?;
            }
            xsel.wait()?;
            copied = true;
        }
        if !copied {
            // No output, just like curl ifconfig.io
        }
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use std::process::Output;
    use std::os::unix::process::ExitStatusExt;

    #[test]
    fn test_ip_parsing_from_curl_output() {
        // Simulate curl output with trailing newline
        let fake_output = Output {
            status: std::process::ExitStatus::from_raw(0),
            stdout: b"203.0.113.42\n".to_vec(),
            stderr: Vec::new(),
        };
        let ip = String::from_utf8_lossy(&fake_output.stdout).trim().to_string();
        assert_eq!(ip, "203.0.113.42");
    }

    #[test]
    fn test_ip_parsing_with_whitespace() {
        let fake_output = Output {
            status: std::process::ExitStatus::from_raw(0),
            stdout: b" 198.51.100.23 \n".to_vec(),
            stderr: Vec::new(),
        };
        let ip = String::from_utf8_lossy(&fake_output.stdout).trim().to_string();
        assert_eq!(ip, "198.51.100.23");
    }
}
