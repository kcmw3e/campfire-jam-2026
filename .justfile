#!/usr/bin/env -S just --justfile

# Make the default recipe just list possible recipes. Taken from the
# `just` documentation:
#     https://github.com/casey/just?tab=readme-ov-file#listing-available-recipes
# 
default:
    @just --list --unsorted --justfile {{justfile()}}

set dotenv-load := true

alias b := build
alias r := run
alias t := test
alias c := clean
alias d := docs
alias od := open-docs

build-dir := env('BUILD_DIR', 'build')
pdf-viewer := env('PDF_VIEWER', '')

docs-target := 'gdd.pdf'
docs-target-filepath := build-dir/docs-target


build:
    cargo build


run:
    cargo run


test:
    cargo test


clean:
    cargo clean


docs: make-build-dir
    typst compile docs/main.typ '{{docs-target-filepath}}'


open-docs in-bg='false': docs
    #!/usr/bin/env fish
    # If `in-bg` is "true" or "t", the docs will be opened with a backgrounded
    # process with its standard output and standard error redirected to
    # /dev/null.

    if test -n '{{pdf-viewer}}'
        if test '{{in-bg}}' = 'true' -o '{{in-bg}}' = 't'
            '{{pdf-viewer}}' '{{docs-target-filepath}}' >/dev/null &
        else
            '{{pdf-viewer}}' '{{docs-target-filepath}}'
        end
    else
        printf ( string join '' \
            'Cannot open docs. Please specify a PDF viewer using the' \
            ' environment variable `PDF_VIEWER`.\n' \
        )
    end


[private]
make-build-dir:
    @mkdir -p '{{build-dir}}/'
