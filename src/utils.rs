use std::{
    io::{self, Read},
    path::PathBuf,
};

use anyhow::{anyhow, Result};
use dirs::data_dir;
use qrcode_generator;

static QRCODE_SIZE: usize = 256;

/// Determine the system's data directory.
pub fn get_data_dir() -> Result<PathBuf> {
    Ok(data_dir()
        .ok_or_else(|| anyhow!("Failed to get default data directory."))?
        .join("thqm"))
}

/// Determine the system's config directory.
// pub fn get_config_dir() -> Result<PathBuf> {
//     Ok(preference_dir()
//         .ok_or_else(|| anyhow!("Failed to get default config directory."))?
//         .join("thqm"))
// }

/// Read stdin.
pub fn read_stdin() -> Result<String> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer)?;
    Ok(buffer)
}

/// Get the local ip.
pub fn get_ip(interface: Option<&str>) -> Result<String> {
    // This currently fails if a "tun" interface is present.
    // There is an issue in the "local_ip_address" crate.
    if let Some(ifa_name) = interface {
        let interfaces = local_ip_address::list_afinet_netifas()?;
        local_ip_address::find_ifa(interfaces, ifa_name)
            .map(|(_, ip)| ip.to_string())
            .ok_or_else(|| anyhow!(format!("Failed to get ip for interface: {:?}", interface)))
    } else {
        Ok(local_ip_address::local_ip().map(|s| s.to_string())?)
    }
}

/// Create the url string.
pub fn create_url(host: &str, port: u64) -> String {
    format!("{host}:{port}", host = host, port = port)
}

/// Create a full url string, with http basic auth if logins provided.
pub fn create_full_url(
    host: &str,
    port: u64,
    username: Option<&str>,
    password: Option<&str>,
) -> String {
    if username.is_some() && password.is_some() {
        format!(
            "http://{username}:{password}@{host}:{port}",
            username = username.unwrap(),
            password = password.unwrap(),
            host = host,
            port = port
        )
    } else {
        format!("http://{host}:{port}", host = host, port = port)
    }
}

/// Construct a qrcode svg string containing the provided `data`.
pub fn create_qrcode_svg_string(data: &str) -> Result<String> {
    Ok(qrcode_generator::to_svg_to_string(
        data,
        qrcode_generator::QrCodeEcc::Low,
        QRCODE_SIZE,
        None::<&str>,
    )?)
}

/// Save a qrcode image to file, containing `data`.
pub fn save_qrcode(data: &str, dest: PathBuf) -> Result<()> {
    Ok(qrcode_generator::to_png_to_file(
        data,
        qrcode_generator::QrCodeEcc::Low,
        QRCODE_SIZE,
        dest,
    )?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_url() {
        assert_eq!(create_url("test_host", 1234), "test_host:1234");
    }

    #[test]
    fn test_create_full_url() {
        assert_eq!(
            create_full_url("test_host", 1234, Some("user"), Some("hunter2")),
            "http://user:hunter2@test_host:1234"
        );
        assert_eq!(
            create_full_url("test_host", 1234, None, Some("hunter2")),
            "http://test_host:1234"
        );
        assert_eq!(
            create_full_url("test_host", 1234, None, None),
            "http://test_host:1234"
        );
    }

    #[test]
    fn test_crate_svg_string() {
        assert!(create_qrcode_svg_string("some data").is_ok());
    }

    #[test]
    fn test_get_ip() {
        assert!(get_ip(None).is_ok());
    }
    #[test]
    fn test_get_ip_missing() {
        assert!(!get_ip(Some("missing_interface")).is_ok());
    }
}
