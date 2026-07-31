#!/usr/bin/env bash

set -euo pipefail

remote=origin
base_tag="snapshot-$(date -u +%Y%m%d)"
tag="$base_tag"
suffix=0

git fetch --quiet --tags "$remote"

while git show-ref --verify --quiet "refs/tags/$tag"; do
  suffix=$((suffix + 1))
  tag="$base_tag-$suffix"
done

git tag -a "$tag" -m "Snapshot $tag"
git push "$remote" "refs/tags/$tag"

printf 'Created and pushed %s\n' "$tag"
