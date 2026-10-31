#!/usr/bin/env bash
# Release guard for re-pushed tags and rewritten history.
#
# crates.io versions are immutable: a published version can be yanked, but its
# number can never be uploaded again. For each workspace crate, this guard asks
# crates.io whether the workspace version already exists and, if it does,
# compares the published .crate with a fresh `cargo package` of the current tree
# (ignoring .cargo_vcs_info.json, which records the git commit and changes on
# every history rewrite, and the Cargo.lock checksums that hash it in turn).
#
#   new         -> not on crates.io yet: publish it.
#   identical   -> already published with the same content (history rewrite or
#                  re-pushed tag): skip it.
#   conflicting -> already published with different content: never re-publish.
#                  With YANK_ON_CONFLICT=true (and DRY_RUN != true) the
#                  conflicting versions are yanked, in reverse dependency order,
#                  with CARGO_REGISTRY_TOKEN (needs the `yank` scope). Either way
#                  the guard exits 1: bump the version in Cargo.toml and tag again.
#
# Exit codes: 0 = safe to publish the `new` crates; 1 = conflict; 2 = registry error.
# Writes `publish=`, `identical=` and `conflicting=` to $GITHUB_OUTPUT when set.
set -euo pipefail

cd "$(dirname "$0")/.."

CRATES=(stochology stochology-cli)
VERSION="$(sed -nE 's/^version = "([^"]+)"/\1/p' Cargo.toml | head -n 1)"
UA="stochology-release-guard (https://github.com/0b3st4f3r4/stochology)"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

new=()
identical=()
conflicting=()
existing=()

for crate in "${CRATES[@]}"; do
    code="$(curl -sS -o /dev/null -w '%{http_code}' -A "$UA" "https://crates.io/api/v1/crates/${crate}/${VERSION}")"
    case "$code" in
        404) new+=("$crate") ;;
        200) existing+=("$crate") ;;
        *)
            echo "ERROR: crates.io answered HTTP ${code} for ${crate}@${VERSION}; cannot decide whether to publish." >&2
            exit 2
            ;;
    esac
done

if [ "${#existing[@]}" -gt 0 ]; then
    # One workspace package run resolves unpublished inter-crate dependencies locally.
    cargo package --workspace --no-verify --allow-dirty --quiet
    for crate in "${existing[@]}"; do
        mkdir -p "$WORK/remote/$crate" "$WORK/local/$crate"
        curl -sSfL -A "$UA" -o "$WORK/$crate.crate" "https://static.crates.io/crates/${crate}/${crate}-${VERSION}.crate"
        tar -xzf "$WORK/$crate.crate" -C "$WORK/remote/$crate"
        tar -xzf "target/package/${crate}-${VERSION}.crate" -C "$WORK/local/$crate"
        # Lockfile checksums of sibling workspace crates hash their .cargo_vcs_info.json too,
        # so they change with the commit; the workspace has no external dependencies.
        find "$WORK/remote/$crate" "$WORK/local/$crate" -name Cargo.lock -exec sed -i '/^checksum = /d' {} +
        if diff -r --exclude=.cargo_vcs_info.json "$WORK/remote/$crate" "$WORK/local/$crate" >"$WORK/$crate.diff"; then
            identical+=("$crate")
        else
            conflicting+=("$crate")
            echo "  ${crate}@${VERSION}: published content differs from this tree:"
            head -n 20 "$WORK/$crate.diff" | sed 's/^/    /'
        fi
    done
fi

echo "Release guard for v${VERSION}:"
echo "  new (will publish):              ${new[*]:-none}"
echo "  identical (already published):   ${identical[*]:-none}"
echo "  conflicting (never re-publish):  ${conflicting[*]:-none}"

if [ -n "${GITHUB_OUTPUT:-}" ]; then
    {
        echo "publish=${new[*]}"
        echo "identical=${identical[*]}"
        echo "conflicting=${conflicting[*]}"
    } >>"$GITHUB_OUTPUT"
fi

if [ "${#conflicting[@]}" -eq 0 ]; then
    exit 0
fi

if [ "${YANK_ON_CONFLICT:-false}" = "true" ] && [ "${DRY_RUN:-false}" != "true" ]; then
    if [ -z "${CARGO_REGISTRY_TOKEN:-}" ]; then
        echo "ERROR: YANK_ON_CONFLICT=true but CARGO_REGISTRY_TOKEN is not set." >&2
        exit 1
    fi
    # Reverse dependency order: dependents first, so no live version points at a yanked one.
    for ((i = ${#CRATES[@]} - 1; i >= 0; i--)); do
        crate="${CRATES[$i]}"
        for c in "${conflicting[@]}"; do
            if [ "$c" = "$crate" ]; then
                echo "  Yanking ${crate}@${VERSION}"
                cargo yank --version "$VERSION" "$crate"
            fi
        done
    done
    echo "Yanked the conflicting versions. crates.io will not accept ${VERSION} again." >&2
elif [ "${YANK_ON_CONFLICT:-false}" = "true" ]; then
    echo "Dry run: would yank ${conflicting[*]} at ${VERSION}." >&2
fi

echo "ERROR: v${VERSION} is already on crates.io with different content. Bump the workspace version in Cargo.toml and tag the new version; enable yanking (YANK_ON_CONFLICT) to retire the old one first." >&2
exit 1
