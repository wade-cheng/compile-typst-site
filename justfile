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
    # prereq
    cargo build

    # docs/how-to/versions.txt
    echo > docs/how-to/versions.txt
    echo "$ typst --version" >> docs/how-to/versions.txt
    typst --version >> docs/how-to/versions.txt
    echo >> docs/how-to/versions.txt
    echo "$ compile-typst-site --version" >> docs/how-to/versions.txt
    ./target/debug/compile-typst-site --version 2>&1 | sed 's|\./target/debug/||g' >> docs/how-to/versions.txt

    # reference apis
    ./target/debug/compile-typst-site --help 2>&1 | sed 's|\./target/debug/||g' > docs/reference/cts_help.txt
    # sed won't work if we have more brackets in the thing.
    sed -n '/^struct ConfigFile/,/^}/p' src/internals/config.rs > docs/reference/config_struct.rs

    # build and serve
    uv pip install -r docs/requirements.txt
    uv run sphinx-autobuild docs docs_out

test-all:
    cargo test --all-features -- --include-ignored 