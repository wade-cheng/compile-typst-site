default:
    just --list --unsorted

[confirm("Check that you've updated cargo.toml and all docs, and the current release pushed to origin is `release: version ...`. Release? [y/N]")]
release tag:
    git tag v{{tag}}
    git push --tags
    cargo publish

# `dist init` can be helpful.

install-locally:
    cargo install --path .

doc:
    uv pip install -r docs/requirements.txt
    uv run sphinx-autobuild docs docs_out

test-all:
    cargo test --all-features -- --include-ignored 