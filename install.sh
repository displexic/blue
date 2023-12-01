#!/bin/bash

# ANSI color codes
RESET='\033[0m' # No color or formatting
GREEN='\033[0;32m'

download() {
  if command -v curl > /dev/null 2>&1; then
    curl -fsSL "$1"
  else
    wget -qO- "$1"
  fi
}


validate_url() {
  local url
  url="$1"

  if command -v curl > /dev/null 2>&1; then
    curl --output /dev/null --silent --show-error --location --head --fail "$url"
  else
    wget --spider --quiet "$url"
  fi
}

is_glibc_compatible() {
  getconf GNU_LIBC_VERSION >/dev/null 2>&1 || ldd --version >/dev/null 2>&1 || return 1
}

detect_platform() {
  local platform
  platform="$(uname -s | tr '[:upper:]' '[:lower:]')"

  case "${platform}" in
    linux)
      if is_glibc_compatible; then
        platform="linux"
      else
        platform="linuxstatic"
      fi
      ;;
    darwin) platform="macos" ;;
    windows) platform="win" ;;
  esac

  printf '%s' "${platform}"
}

detect_arch() {
  local arch
  arch="$(uname -m | tr '[:upper:]' '[:lower:]')"

  case "${arch}" in
    x86_64 | amd64) arch="amd64" ;;
    armv*) arch="arm" ;;
    arm64 | aarch64) arch="arm64" ;;
  esac

  # `uname -m` in some cases mis-reports 32-bit OS as 64-bit, so double check
  if [ "${arch}" = "amd64" ] && [ "$(getconf LONG_BIT)" -eq 32 ]; then
    arch=i686
  elif [ "${arch}" = "arm64" ] && [ "$(getconf LONG_BIT)" -eq 32 ]; then
    arch=arm
  fi

  case "$arch" in
    amd64*) ;;
    arm64*) ;;
    *) return 1
  esac
  printf '%s' "${arch}"
}

get_latest_release_tag(){
  res=$(curl -s "https://api.github.com/repos/slekup/blue/releases")
  tag=$(echo $res | grep -o '"tag_name": *"[^"]*"' | awk -F'"' '{print $4}')
  echo "$tag"
}

download_and_install() {
  local platform arch download file tag tmp_dir
  platform="$(detect_platform)"
  arch="$(detect_arch)" || abort "Sorry! Blue currently only provides pre-built binaries for x86_64/arm64 architectures."
  file="blue-${platform}-${arch}"
  tag=$(get_latest_release_tag)
  if [ "${platform}" = "win" ]; then
    file="${file}.exe"
  fi
  download="https://github.com/slekup/blue/releases/download/$tag/$file"

  validate_url "$download"  || abort "Blue version '${tag}' could not be found"

    # install to BLUE_HOME, defaulting to ~/.blue
  tmp_dir="$(mktemp -d)" || abort "Tmpdir Error!"
  trap 'rm -rf "$tmp_dir"' EXIT INT TERM HUP

  echo -e "${GREEN}Downloading Blue from latest GitHub release...${RESET}\n"

  # download the binary to the specified directory
  download "$download" > "$tmp_dir/blue"  || return 1
  chmod +x "$tmp_dir/blue"
  SHELL="$SHELL" "$tmp_dir/blue" setup || return 1
}

download_and_install || abort "Install Error!"



