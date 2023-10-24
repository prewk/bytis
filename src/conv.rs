use std::collections::HashMap;

use git2::Commit;

use crate::data::{Bump, Config, IdConventionalCommit};

pub fn determine_bump(grouped: &HashMap<String, Vec<IdConventionalCommit>>) -> Bump {
    let mut bump = Bump::Patch;

    for (_, commits) in grouped {
        for commit in commits {
            if bump == Bump::Major || commit.is_breaking {
                bump = Bump::Major;
            } else if bump == Bump::Minor || commit.conv.type_() == "feat" {
                bump = Bump::Minor;
            }
        }
    }

    bump
}

pub fn parse_into_grouped(
    config: &Config,
    commits: &Vec<Commit>,
) -> HashMap<String, Vec<IdConventionalCommit>> {
    let fallback = "".to_string();

    commits
        .iter()
        .map(IdConventionalCommit::from_commit)
        .filter_map(Result::ok)
        .map(|commit| {
            (
                config
                    .headings
                    .get(commit.conv.type_())
                    .unwrap_or(&fallback),
                commit,
            )
        })
        .fold(HashMap::new(), |mut acc, (_, commit)| {
            let grouped = acc.entry(commit.conv.type_().to_string()).or_insert(vec![]);

            grouped.push(commit);

            acc
        })
}
