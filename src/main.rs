extern crate conventional_commit;
extern crate git2;
extern crate semver;

use std::error::Error;
use std::process;

use clap::{Parser, Subcommand};
use git2::Repository;
use semver::Version;

use conv::*;
use data::*;
use format::*;
use repo::*;

mod conv;
mod data;
mod format;
mod repo;

#[derive(Debug, Subcommand)]
enum Output {
    /// prints next version
    NextVersion,
    /// prints markdown changelog
    Markdown,
    /// prints mrkdwn changelog
    Mrkdwn,
}

#[derive(Parser)]
#[command(author = "Oskar Thornblad", version = "0.0.1")]
struct Cli {
    #[command(subcommand)]
    output: Output,

    /// Specify type-to-headings for the changelog, ie. -H "feat=Features" -H "fix=Fixes"
    #[arg(short = 'H', long, value_parser = parse_key_val, value_name = "type=heading")]
    heading: Vec<(String, String)>,

    /// Specify commit URL template with {id}, ie. https://example.com/foo/bar/commmit/{id}
    #[arg(short = 'U', long)]
    commit_url: Option<String>,
}

/// Parse a single key-value pair
fn parse_key_val(s: &str) -> Result<(String, String), Box<dyn Error + Send + Sync + 'static>> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].to_string(), s[pos + 1..].to_string()))
}

fn run(output: Output, config: &Config) -> Result<String, String> {
    let repo = Repository::open(
        std::env::current_dir().or(Err("Failed to determine current dir".to_string()))?,
    )
    .or(Err("Failed to open repository".to_string()))?;

    let latest_version = get_latest_version(&repo).ok_or(&"No semver tags detected".to_string())?;

    let commits = get_commits_from_tag_to_head(&repo, &latest_version).unwrap();

    let grouped = parse_into_grouped(&config, &commits);

    let bump = determine_bump(&grouped);

    let next_version = match bump {
        Bump::Major => Version::new(latest_version.major + 1, 0, 0),
        Bump::Minor => Version::new(latest_version.major, latest_version.minor + 1, 0),
        Bump::Patch => Version::new(
            latest_version.major,
            latest_version.minor,
            latest_version.patch + 1,
        ),
    };

    match output {
        Output::NextVersion => Ok(next_version.to_string()),
        Output::Markdown => Ok(create_markdown_changelog(
            &next_version.to_string(),
            &grouped,
            &config,
        )),
        Output::Mrkdwn => Ok(create_mrkdwn_changelog(&grouped, &config)),
    }
}

fn main() {
    let cli = Cli::parse();

    let config = Config {
        headings: cli.heading.into_iter().collect(),
        repo_commit_url_tpl: cli.commit_url,
    };

    let success = run(cli.output, &config).unwrap_or_else(|e| {
        eprint!("Error: {}", e);
        process::exit(1);
    });

    print!("{}", success);
}
