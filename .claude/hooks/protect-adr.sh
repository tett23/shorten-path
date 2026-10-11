#!/bin/sh
# PreToolUse hook: rejects Edit/Write on committed ADRs unless only the status
# line changes.
set -eu

input=$(cat)
tool=$(printf '%s' "$input" | jq -r '.tool_name')
file=$(printf '%s' "$input" | jq -r '.tool_input.file_path // empty')

cd "${CLAUDE_PROJECT_DIR:-.}"
root=$(git rev-parse --show-toplevel)
rel=${file#"$root"/}

case "$rel" in
  docs/adr/*.md) ;;
  *) exit 0 ;;
esac

if ! git cat-file -e "HEAD:${rel}" 2>/dev/null; then
  exit 0
fi

strip_status() {
  grep -v '^ステータス:' || true
}

case "$tool" in
  Write)
    before=$(git show "HEAD:${rel}" | strip_status)
    after=$(printf '%s' "$input" | jq -r '.tool_input.content' | strip_status)
    ;;
  Edit)
    before=$(printf '%s' "$input" | jq -r '.tool_input.old_string' | strip_status)
    after=$(printf '%s' "$input" | jq -r '.tool_input.new_string' | strip_status)
    ;;
  *) exit 0 ;;
esac

if [ "$before" != "$after" ]; then
  echo "$rel is a committed ADR. Only the 'ステータス:' line may be changed; write a new ADR instead." >&2
  exit 2
fi
