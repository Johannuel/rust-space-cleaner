#!/usr/bin/env bash
# Rebuild rust-space-cleaner agent tmux layout from scratch.
#
#   tools/agents-tmux.sh rebuild   # kill everything + recreate 5 windows
#   tools/agents-tmux.sh attach    # attach to session 0 (Ctrl-b n next window)
#
# Windows on session 0:
#   0: s4  -> repo main      (integrador, este agente)
#   1: s1  -> /tmp/rsc-s1    (clean batch + state)
#   2: s2  -> /tmp/rsc-s2    (TUI v2 / progress mpsc)
#   3: s3  -> /tmp/rsc-s3    (scan + fuentes + fixtures)
#   4: s4b -> /tmp/rsc-s4-bench (benchs, opcional)
set -euo pipefail

REPO=/home/mfcode/Projects/rust-space-cleaner

case "${1:-}" in
  up|rebuild)
    echo "==> killing all tmux servers"
    tmux kill-server 2>/dev/null || true
    sleep 1

    echo "==> ensuring worktrees exist"
    git -C "$REPO" worktree add /tmp/rsc-s1 -b feat/clean-batch 2>/dev/null || true
    git -C "$REPO" worktree add /tmp/rsc-s2 feat/progress-tui 2>/dev/null || true
    git -C "$REPO" worktree add /tmp/rsc-s3 feat/scan-portable 2>/dev/null || true
    git -C "$REPO" worktree add /tmp/rsc-s4-bench feat/bench 2>/dev/null || true

    echo "==> creating session 0 (detached) with 5 windows"
    tmux new-session -d -s 0 -n s4 -c "$REPO"
    tmux new-window  -t 0 -n s1 -c /tmp/rsc-s1
    tmux new-window  -t 0 -n s2 -c /tmp/rsc-s2
    tmux new-window  -t 0 -n s3 -c /tmp/rsc-s3
    tmux new-window  -t 0 -n s4b -c /tmp/rsc-s4-bench

    echo "==> launching opencode in each window"
    tmux send-keys -t 0:s4 'opencode' Enter
    tmux send-keys -t 0:s1 'opencode' Enter
    tmux send-keys -t 0:s2 'opencode' Enter
    tmux send-keys -t 0:s3 'opencode' Enter
    tmux send-keys -t 0:s4b 'opencode' Enter

    echo "==> done. Attach with: tmux attach -t 0  (windows: 0-4, Ctrl-b C to switch)"
    ;;
  attach)
    tmux attach -t 0
    ;;
  *)
    echo "usage: $0 {rebuild|attach}" >&2
    exit 1
    ;;
esac