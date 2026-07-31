#!/usr/bin/env bash

set -euo pipefail

remote=origin
date_stamp="$(date -u +%Y%m%d)"
main_base_tag="keyman-snapshot-$date_stamp"
cli_base_tag="keyman-cli-snapshot-$date_stamp"
main_tag="$main_base_tag"
cli_tag="$cli_base_tag"
suffix=0

git fetch --quiet --tags "$remote"

while git show-ref --verify --quiet "refs/tags/$main_tag" ||
      git show-ref --verify --quiet "refs/tags/$cli_tag"; do
  suffix=$((suffix + 1))
  main_tag="$main_base_tag-$suffix"
  cli_tag="$cli_base_tag-$suffix"
done

git tag -a "$main_tag" -m "Snapshot $main_tag"
git tag -a "$cli_tag" -m "Snapshot $cli_tag"
git push --atomic "$remote" "refs/tags/$main_tag" "refs/tags/$cli_tag"

printf 'Created and pushed %s\n' "$main_tag"
printf 'Created and pushed %s\n' "$cli_tag"
