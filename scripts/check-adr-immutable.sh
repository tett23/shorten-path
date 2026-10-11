#!/bin/sh
# Fails when an ADR committed in BASE was changed in TARGET in anything other
# than its status line, or was deleted/renamed.
#
# usage: scripts/check-adr-immutable.sh [BASE] [TARGET]
#   BASE   defaults to HEAD
#   TARGET defaults to the index (staged contents)
set -eu

base=${1:-HEAD}
target=${2:-}

if ! git rev-parse --verify --quiet "${base}^{commit}" >/dev/null; then
  exit 0
fi

strip_status() {
  grep -v '^ステータス:' || true
}

status=0
git ls-tree --name-only "$base" -- docs/adr/ | grep '\.md$' | {
  while IFS= read -r path; do
    if ! git cat-file -e "${target}:${path}" 2>/dev/null; then
      echo "ADR deleted or renamed: $path" >&2
      status=1
      continue
    fi

    before=$(git show "${base}:${path}" | strip_status)
    after=$(git show "${target}:${path}" | strip_status)
    if [ "$before" != "$after" ]; then
      echo "ADR changed outside its status line: $path" >&2
      status=1
    fi
  done

  if [ "$status" -ne 0 ]; then
    echo "Committed ADRs are immutable. Write a new ADR instead." >&2
  fi
  exit "$status"
}
