use clap::{CommandFactory, FromArgMatches, Parser};
use get_3gpp_spec::{DateFilter, SpecNumber};
use std::fs;
use std::io::copy;
use std::path::{Path, PathBuf};

mod config;

fn download_url_to_path(url: &str, dest: &Path) -> Result<PathBuf, String> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("failed to create directory '{}': {}", parent.display(), e))?;
    }

    let resp =
        reqwest::blocking::get(url).map_err(|e| format!("request failed for '{}': {}", url, e))?;

    if !resp.status().is_success() {
        return Err(format!(
            "failed to download '{}': status {}",
            url,
            resp.status()
        ));
    }

    let content = resp
        .bytes()
        .map_err(|e| format!("failed to read response body for '{}': {}", url, e))?;

    let mut file = fs::File::create(dest)
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

    /// Output format for --list
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Text)]
    output: OutputFormat,
}

/// Output format for listing specs.
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

/// Build the clap command, augmenting `--help` with the resolved config paths.
fn build_command() -> clap::Command {
    let settings_display = config::settings_path()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "<unavailable>".to_string());
    let destination = config::resolve_destination();

    Args::command().after_help(format!(
        "Download configuration:\n  Settings file: {}\n  Destination:   {}\n\nTo change where files are saved, create or edit the settings file\nwith a single line:\n  destination = \"<path of desired folder>\"",
        settings_display,
        destination.display()
    ))
}

fn main() {
    let args = Args::from_arg_matches(&build_command().get_matches()).unwrap_or_else(|e| e.exit());
    match get_3gpp_spec::list(args.spec_number, args.release, args.date) {
        Ok(items) => {
            match args.list {
                false => {
                    if let Some(item) = items.first() {
                        // Determine filename from URL path segment
                        let filename = match reqwest::Url::parse(&item.url).ok().and_then(|u| {
                            u.path_segments()
                                .and_then(|s| s.last())
                                .map(|s| s.to_string())
                        }) {
                            Some(f) if !f.is_empty() => f,
                            _ => "download.bin".to_string(),
                        };

                        let download_dir = config::resolve_destination();

                        let dest = download_dir.join(&filename);

                        match download_url_to_path(&item.url, &dest) {
                            Ok(path) => {
                                println!("downloaded to {}", path.display());
                            }
                            Err(e) => eprintln!("{}", e),
                        }
                    } else {
                        eprintln!("no matching item found");
                    }
                    return;
                }
                true => match args.output {
                    OutputFormat::Text => {
                        for item in items.iter() {
                            println!("{}", item);
                        }
                    }
                    OutputFormat::Json => match serde_json::to_string_pretty(&items) {
                        Ok(json) => println!("{}", json),
                        Err(e) => eprintln!("failed to serialize items to JSON: {}", e),
                    },
                },
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}
