#!/usr/bin/env bash
# Run helper for docker-compose driven development of Code-Realm-TS
set -euo pipefail

# Always execute in the repository root where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd "$SCRIPT_DIR"

service="code-realm" # docker compose service name

# Prefer Docker Compose v2 ('docker compose'); fall back to legacy 'docker-compose'
if docker compose version >/dev/null 2>&1; then
  COMPOSE_CMD="docker compose"
elif command -v docker-compose >/dev/null 2>&1 && docker-compose version >/dev/null 2>&1; then
  COMPOSE_CMD="docker-compose"
else
  echo "Error: Docker Compose not found (need 'docker compose' or 'docker-compose')" >&2
  exit 1
fi

########################################
# Helper functions
########################################

# Print help text
show_help() {
  cat <<EOF
Usage: ./run_docker.sh <command>

Commands:
  start        Build and start the containers (interactive)
  play         Launch the game in an interactive session (ephemeral container)
  stop         Stop and remove containers, networks, volumes (docker-compose down)
  test         Run Jest test suite inside container
  grader       Run quest grader tests (npm run test:grader)
  shell        Spawn an interactive Bash shell inside the container
  help         Show this help message
  <other>      Any other arguments are passed directly to docker-compose
EOF
}

# Execute a recognised sub-command
run_cmd() {
  local cmd="$1"; shift || true
  case "$cmd" in
    start)
      $COMPOSE_CMD up --build "$@";
      ;;
    play)
      # Run the service in an ephemeral container with ports & TTY so stdin works
      $COMPOSE_CMD run --rm --service-ports $service "$@";
      ;;
    stop)
      $COMPOSE_CMD down "$@";
      ;;
    test)
      $COMPOSE_CMD run --rm $service npm test "$@";
      ;;
    grader)
      $COMPOSE_CMD run --rm $service npm run test:grader "$@";
      ;;
    shell)
      $COMPOSE_CMD run --rm --entrypoint sh $service "$@";
      ;;
    help|-h|--help)
      show_help;
      ;;
    *)
      # Forward everything else straight to docker-compose
      $COMPOSE_CMD "$cmd" "$@";
      ;;
  esac
}

# Interactive menu using Bash 'select'
show_menu() {
  while true; do
    cat <<EOF

========================================
 Docker Compose Menu
========================================
 1) Play game (interactive)
 2) Run Jest tests
 3) Run quest grader
 4) Open shell inside container
 5) Other docker-compose command
 6) Quit (or q)
+----------------------------------------
EOF

    read -rp "Enter choice (1-6, q to quit): " choice

    case "${choice}" in
      1) run_cmd play ;;
      2) run_cmd test ;;
      3) run_cmd grader ;;
      4) run_cmd shell ;;
      5)
         read -rp "$COMPOSE_CMD " extra
         # shellcheck disable=SC2086 # word splitting desired
         $COMPOSE_CMD $extra ;;
      6|q|Q) cleanup; break ;;
      *) echo "Invalid option: ${choice}" ;;
    esac
  done
}

# Bring down containers/networks
cleanup() {
  echo "Shutting down containers..."
  $COMPOSE_CMD down >/dev/null 2>&1 || true
}

#---------------------------------------
# Main entry
#---------------------------------------

if [[ $# -eq 0 || "$1" == "menu" ]]; then
  show_menu
  exit 0
fi

# Preserve existing non-interactive behaviour
cmd="$1"; shift || true
run_cmd "$cmd" "$@" 