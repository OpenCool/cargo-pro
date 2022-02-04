use anyhow::Result;
use git2::{
    Repository, Signature, Commit,
};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
// use cached::proc_macro::cached;

pub struct Repo<'a> {
    pub repo: &'a Repository,
    pub cache: Cache<'a>,
}

pub struct Cache<'a> {
    pub commits: Vec<Commit<'a>>,
    pub logs: Vec<CommitLog>,
}

// https://github.com/rust-lang/git2-rs/blob/master/src/mailmap.rs
#[derive(Hash, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Author {
    pub name: String,
    pub email: String,
    pub seconds: i64,
}

impl From<Signature<'_>> for Author {
    fn from(sig: Signature) -> Self {
        let name = String::from_utf8_lossy(sig.name_bytes()).into_owned();
        let email = String::from_utf8_lossy(sig.email_bytes()).into_owned();
        let seconds = sig.when().seconds();
        Self { name, email, seconds }
    }
}

#[derive(Hash, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CommitLog {
    pub name: String,
    pub email: String,
    pub seconds: i64,
    pub message: String,
}

impl From<Commit<'_>> for CommitLog {
    fn from(commit: Commit<'_>) -> Self {
        let name = String::from_utf8_lossy(commit.committer().name_bytes()).into_owned();
        let email = String::from_utf8_lossy(commit.committer().email_bytes()).into_owned();
        let message = String::from_utf8_lossy(commit.message_bytes()).into_owned();
        let seconds = commit.time().seconds();
        Self { name, email, seconds, message }
    }
}

impl<'a> Repo<'a> {
    pub fn new(repo: &'a Repository) -> Result<Self> {
        let commits = Self::commits(&repo)?;
        let logs = Self::logs(&commits)?;
        Ok(Self {
            repo,
            cache: Cache { commits, logs },
        })
    }


    // #[cached]
    fn commits(
        repo: &'a Repository,
    ) -> Result<Vec<Commit<'a>>> {
        // https://github.com/rust-lang/git2-rs/blob/master/src/revwalk.rs
        let mut revwalk = repo.revwalk()?;
        // Push the repository's HEAD
        revwalk.push_head()?;
        let logs: Vec<Commit<'a>> = revwalk
            .filter_map(|r| match r {
                Err(_) => None,
                Ok(r) => repo
                    .find_commit(r)
                    .ok(),
            })
            .collect();

        Ok(logs)
    }

    // #[cached]
    pub fn logs(
        commit_list: &Vec<Commit<'a>>,
    ) -> Result<Vec<CommitLog>> {
        let mut commitlogs: Vec<CommitLog> = vec![];
        for commit in commit_list {
            commitlogs.push(CommitLog::from(commit.to_owned()));
        }
        Ok(commitlogs)
    }
}