use clap::Parser;
use get_3gpp_spec::{SpecNumber, DateFilter};
use std::fs::File;
use std::io::copy;
use std::path::{Path, PathBuf};

fn download_url_to_path(url: &str, dest: &Path) -> Result<PathBuf, String> {
    let resp = reqwest::blocking::get(url)
        .map_err(|e| format!("request failed for '{}': {}", url, e))?;

    if !resp.status().is_success() {
        return Err(format!("failed to download '{}': status {}", url, resp.status()));
    }

    let content = resp
        .bytes()
        .map_err(|e| format!("failed to read response body for '{}': {}", url, e))?;

    let mut file = File::create(dest)
        .map_err(|e| format!("failed to create file '{}': {}", dest.display(), e))?;

    copy(&mut content.as_ref(), &mut file)
        .map_err(|e| format!("failed to write to '{}': {}", dest.display(), e))?;

    Ok(dest.to_path_buf())
}

/// Simple CLI for fetching 3GPP spec info
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 3GPP spec number (positional)
    spec_number: SpecNumber,

    /// Date string (optional) — format must be YYYY-MM
    #[arg(short, long)]
    date: Option<DateFilter>,

    /// Release number (nonnegative integer)
    #[arg(short, long, value_parser = clap::value_parser!(u32))]
    release: Option<u32>,

    /// List flag (default: false)
    #[arg(short, long, default_value_t = false)]
    list: bool,
}

fn main() {
    let args = Args::parse();
    match get_3gpp_spec::list(args.spec_number, args.release, args.date) {
        Ok(items) => {
            match args.list {
                false => {
                    if let Some(item) = items.first() {
                        // Determine filename from URL path segment
                        let filename = match reqwest::Url::parse(&item.url)
                            .ok()
                            .and_then(|u| u.path_segments().and_then(|s| s.last()).map(|s| s.to_string()))
                        {
                            Some(f) if !f.is_empty() => f,
                            _ => "download.bin".to_string(),
                        };

                        let dest = Path::new(&filename);

                        match download_url_to_path(&item.url, dest) {
                            Ok(path) => println!("downloaded to {}", path.display()),
                            Err(e) => eprintln!("{}", e),
                        }
                    } else {
                        eprintln!("no matching item found");
                    }
                    return;
                }
                true => {
                    for item in items.iter() {
                        println!("{}", item);
                    }
                }
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}
