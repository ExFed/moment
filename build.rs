use git2::{DescribeFormatOptions as DescFmtOpts, DescribeOptions as DescOpts};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = git2::Repository::discover(".")?;
    let head = repo.head()?;
    let commit = head.peel_to_commit()?;

    emit_env("COMMIT_HASH", &commit.id().to_string());
    emit_env("COMMIT_DATE", &commit.time().seconds().to_string());
    emit_env(
        "DESCRIBE_VERSION",
        &repo
            .describe(
                DescOpts::new()
                    .describe_tags()
                    .pattern("v*")
                    .show_commit_oid_as_fallback(true),
            )?
            .format(Some(DescFmtOpts::new().dirty_suffix("*")))?,
    );

    Ok(())
}

fn emit_env(key: &str, value: &str) {
    println!("cargo:rustc-env={}={}", key, value);
}
