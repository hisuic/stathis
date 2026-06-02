#!/bin/sh
set -eu

RELEASE_URL="${STATHIS_RELEASE_URL:-https://github.com/hisuic/stathis/releases/download/latest/stathis}"
DEPLOY_DIR="${STATHIS_DEPLOY_DIR:-/home/murray/builds/stathis}"
SERVICE_NAME="${STATHIS_SERVICE_NAME:-stathis}"
RESTART_SERVICE=1

usage() {
  cat <<EOF
Usage: $0 [--no-restart]

Download the latest Stathis Raspberry Pi binary from GitHub Releases and place
it at:

  ${DEPLOY_DIR}/stathis

Environment variables:
  STATHIS_RELEASE_URL   Release asset URL
  STATHIS_DEPLOY_DIR    Destination directory
  STATHIS_SERVICE_NAME  systemd service name

Options:
  --no-restart          Do not restart the systemd service after deployment
  -h, --help            Show this help
EOF
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --no-restart)
      RESTART_SERVICE=0
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown option: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
  shift
done

mkdir -p "$DEPLOY_DIR"

tmp_file="$(mktemp "${DEPLOY_DIR}/stathis.XXXXXX")"
cleanup() {
  rm -f "$tmp_file"
}
trap cleanup EXIT INT TERM

echo "Downloading latest Stathis binary..."
echo "  from: $RELEASE_URL"
echo "  to:   ${DEPLOY_DIR}/stathis"

if command -v curl >/dev/null 2>&1; then
  curl -fL -o "$tmp_file" "$RELEASE_URL"
elif command -v wget >/dev/null 2>&1; then
  wget -O "$tmp_file" "$RELEASE_URL"
else
  echo "curl or wget is required." >&2
  exit 1
fi

if [ ! -s "$tmp_file" ]; then
  echo "Downloaded file is empty." >&2
  exit 1
fi

chmod +x "$tmp_file"
mv "$tmp_file" "${DEPLOY_DIR}/stathis"
trap - EXIT INT TERM

echo "Deployed: ${DEPLOY_DIR}/stathis"

if [ "$RESTART_SERVICE" -eq 1 ]; then
  if ! command -v systemctl >/dev/null 2>&1; then
    echo "systemctl was not found; skipping service restart."
    exit 0
  fi

  echo "Restarting systemd service: ${SERVICE_NAME}"
  if [ "$(id -u)" -eq 0 ]; then
    systemctl restart "$SERVICE_NAME"
  else
    sudo systemctl restart "$SERVICE_NAME"
  fi

  echo "Service restarted. Check status with:"
  echo "  systemctl status ${SERVICE_NAME}"
fi
