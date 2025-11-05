default:
    just --list --unsorted

[confirm("Check that you've updated cargo.toml and all docs, and the current release pushed to origin is `release: version ...`. Release? [y/N]")]
release tag:
    git tag v{{tag}}
    git push --tags

# `dist init` can be helpful.
