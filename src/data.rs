use std::collections::HashMap;
use std::str::FromStr;

use conventional_commit::ConventionalCommit;
use git2::{Commit, Oid};

#[derive(Debug)]
pub struct IdConventionalCommit {
    pub id: Oid,
    pub conv: ConventionalCommit,
    pub is_breaking: bool,
}

impl IdConventionalCommit {
    pub fn from_commit(
        commit: &Commit,
    ) -> Result<IdConventionalCommit, conventional_commit::Error> {
        ConventionalCommit::from_str(commit.message().unwrap_or("")).map(|conv| {
            IdConventionalCommit {
                id: commit.id().clone(),
                is_breaking: conv.type_().ends_with("!") || conv.breaking_change().is_some(),
                conv,
            }
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum Bump {
    Major,
    Minor,
    Patch,
}

#[derive(Debug)]
pub struct Config {
    pub headings: HashMap<String, String>,
    pub repo_commit_url_tpl: Option<String>,
}
