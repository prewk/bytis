use std::collections::HashMap;

use crate::data::{Config, IdConventionalCommit};

pub fn create_markdown_changelog(
    grouped: &HashMap<String, Vec<IdConventionalCommit>>,
    config: &Config,
) -> String {
    let mut output = String::new();

    for (_type, commits) in grouped {
        let sub_heading = config.headings.get(_type);

        if sub_heading.is_none() {
            continue;
        } else {
            output.push_str(&format!("\n## {}\n\n", sub_heading.unwrap()));
        }

        for commit in commits {
            let id = commit.id.to_string();

            match &config.repo_commit_url_tpl {
                None => output.push_str(&format!(
                    "* {}{} ({})\n",
                    commit
                        .conv
                        .scope()
                        .map(|scope| format!("**{}**: ", scope))
                        .unwrap_or("".to_string()),
                    commit.conv.description(),
                    id.chars().into_iter().take(8).collect::<String>(),
                )),
                Some(repo_commit_url_tpl) => output.push_str(&format!(
                    "* {}{} ([{}]({}))\n",
                    commit
                        .conv
                        .scope()
                        .map(|scope| format!("**{}**: ", scope))
                        .unwrap_or("".to_string()),
                    commit.conv.description(),
                    id.chars().into_iter().take(8).collect::<String>(),
                    str::replace(repo_commit_url_tpl, "{id}", &id.to_string())
                )),
            }
        }
    }

    output.to_owned()
}

pub fn create_mrkdwn_changelog(
    grouped: &HashMap<String, Vec<IdConventionalCommit>>,
    config: &Config,
) -> String {
    let mut output = String::new();

    for (_type, commits) in grouped {
        let sub_heading = config.headings.get(_type);

        if sub_heading.is_none() {
            continue;
        } else {
            output.push_str(&format!("{}\n", sub_heading.unwrap()));
        }

        for commit in commits {
            let id = commit.id.to_string();

            match &config.repo_commit_url_tpl {
                None => output.push_str(&format!(
                    "• {}{} ({})\n",
                    commit
                        .conv
                        .scope()
                        .map(|scope| format!("*{}*: ", scope))
                        .unwrap_or("".to_string()),
                    commit.conv.description(),
                    id.chars().into_iter().take(8).collect::<String>(),
                )),
                Some(repo_commit_url_tpl) => output.push_str(&format!(
                    "• {}{} (<{}|{}>)\n",
                    commit
                        .conv
                        .scope()
                        .map(|scope| format!("*{}*: ", scope))
                        .unwrap_or("".to_string()),
                    commit.conv.description(),
                    id.chars().into_iter().take(8).collect::<String>(),
                    str::replace(repo_commit_url_tpl, "{id}", &id.to_string())
                )),
            }
        }
    }

    output.to_owned()
}
