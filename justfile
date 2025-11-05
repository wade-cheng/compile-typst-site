default:
    just --list --unsorted

[confirm("Check that you've updated cargo.toml and all docs. Release? [y/N]")]
release tag:
    git tag v{{tag}}
    git push --tags

# `dist init` can be helpful.
