use git2::Repository;

use crate::domain::Commit;

pub(crate) fn indy_jones_that_repo(repo: Repository) -> Result<Vec<Commit>, git2::Error> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;

    let mut commit_list: Vec<Commit> = Vec::new();
    for oid in revwalk {
        let commit = repo.find_commit(oid?)?;
        commit_list.push(Commit {
            message: commit.summary().unwrap_or("").into(),
            author: commit.author().name().unwrap_or("").into(),
        });
    }
    Ok(commit_list)
}
