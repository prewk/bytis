use git2::{Commit, Error, Oid, Repository};
use semver::Version;

pub fn get_commits_from_tag_to_head<'r>(
    repo: &'r Repository,
    latest_version: &Version,
) -> Result<Vec<Commit<'r>>, Error> {
    let mut commits = vec![];

    let tag = repo.find_reference(&format!("refs/tags/{}", latest_version.to_string()))?;

    let tag_commit = tag.peel_to_commit()?;

    let mut revwalk = repo.revwalk().expect("Failed to create revwalk");

    revwalk.push(repo.head()?.target().unwrap_or(Oid::zero()))?;

    for oid in revwalk {
        let commit = repo.find_commit(oid?)?;

        if commit.id() == tag_commit.id() {
            break;
        }

        commits.push(commit);
    }

    commits.reverse();

    Ok(commits)
}

/// Get the latest version from the repo tags
pub fn get_latest_version(repo: &Repository) -> Option<Version> {
    // Get the tags
    let tags = repo.tag_names(None).ok()?;
    // Prepare a versions vector
    let mut versions = Vec::new();

    // Iterate through the tags
    for tag in tags.iter() {
        let name = tag?;
        // If a version tag is detected, push it to the versions vector
        match Version::parse(name) {
            Ok(parsed) => versions.push(parsed),
            _ => {}
        }
    }

    // Were there any versions at all?
    if !versions.is_empty() {
        // Yep, sort them
        versions.sort();
        // Get the highest
        let highest_version = versions.last()?;
        // Return a clone of it
        Some(highest_version.clone())
    } else {
        // Nope
        None
    }
}
