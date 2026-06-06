#!/bin/sh
# coffeebreak installer for Unix (Linux / macOS).
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/j-pfalzgraf/coffeebreak/main/install.sh | sh
#
# Environment overrides:
#   COFFEEBREAK_VERSION       Release tag to install (e.g. v0.1.0). Default: latest.
#   COFFEEBREAK_INSTALL_DIR   Directory to install into. Default: $HOME/.local/bin
#
# This script:
#   - detects the platform target triple,
#   - downloads the matching release tarball plus the SHA256SUMS file,
#   - verifies the tarball's sha256 against SHA256SUMS BEFORE extracting,
#   - installs the `coffeebreak` binary, and
#   - prints a PATH hint if the install dir is not on $PATH.
#
# POSIX sh only (it is piped to `sh`, which may be dash): no bashisms.

set -eu

# ----------------------------------------------------------------------------
# Constants
# ----------------------------------------------------------------------------
REPO_OWNER="j-pfalzgraf"
REPO_NAME="coffeebreak"
BIN_NAME="coffeebreak"
GH_RELEASES="https://github.com/${REPO_OWNER}/${REPO_NAME}/releases"

# ----------------------------------------------------------------------------
# Helpers
# ----------------------------------------------------------------------------

# Print a message to stderr (so stdout stays clean for any piping).
log() {
	printf '%s\n' "$*" >&2
}

# Print an error message and exit non-zero.
err() {
	printf 'error: %s\n' "$*" >&2
	exit 1
}

# Does a command exist?
have() {
	command -v "$1" >/dev/null 2>&1
}

# ----------------------------------------------------------------------------
# Platform detection -> Rust target triple
# ----------------------------------------------------------------------------
detect_target() {
	os="$(uname -s)"
	arch="$(uname -m)"

	# Normalise the architecture.
	case "$arch" in
		x86_64 | amd64) arch="x86_64" ;;
		aarch64 | arm64) arch="aarch64" ;;
		*) err "unsupported architecture: ${arch}" ;;
	esac

	# Map OS + arch to a supported target triple.
	case "$os" in
		Linux) TARGET="${arch}-unknown-linux-gnu" ;;
		Darwin) TARGET="${arch}-apple-darwin" ;;
		*) err "unsupported operating system: ${os} (this installer supports Linux and macOS)" ;;
	esac
}

# ----------------------------------------------------------------------------
# Download a URL to a file: prefer curl, fall back to wget.
# Usage: download <url> <dest>
# ----------------------------------------------------------------------------
download() {
	_url="$1"
	_dest="$2"
	if have curl; then
		# -f: fail on HTTP errors, -S: show errors, -s: silent, -L: follow redirects.
		curl -fSsL "$_url" -o "$_dest"
	elif have wget; then
		wget -q "$_url" -O "$_dest"
	else
		err "need either 'curl' or 'wget' installed to download files"
	fi
}

# ----------------------------------------------------------------------------
# Compute the sha256 of a file, printing just the 64-hex digest.
# Prefer sha256sum (Linux), fall back to `shasum -a 256` (macOS).
# ----------------------------------------------------------------------------
sha256_of() {
	_file="$1"
	if have sha256sum; then
		sha256sum "$_file" | awk '{print $1}'
	elif have shasum; then
		shasum -a 256 "$_file" | awk '{print $1}'
	else
		err "need either 'sha256sum' or 'shasum' to verify the download"
	fi
}

# ----------------------------------------------------------------------------
# Main
# ----------------------------------------------------------------------------
main() {
	detect_target

	# Resolve version + source URLs. Default is the "latest" release.
	VERSION="${COFFEEBREAK_VERSION:-latest}"
	ASSET="${BIN_NAME}-${TARGET}.tar.gz"

	if [ "$VERSION" = "latest" ]; then
		ASSET_URL="${GH_RELEASES}/latest/download/${ASSET}"
		SUMS_URL="${GH_RELEASES}/latest/download/SHA256SUMS"
	else
		ASSET_URL="${GH_RELEASES}/download/${VERSION}/${ASSET}"
		SUMS_URL="${GH_RELEASES}/download/${VERSION}/SHA256SUMS"
	fi

	# Resolve install dir.
	INSTALL_DIR="${COFFEEBREAK_INSTALL_DIR:-$HOME/.local/bin}"

	# Private temp dir, cleaned up on exit/interrupt.
	TMPDIR_CB="$(mktemp -d 2>/dev/null || mktemp -d -t coffeebreak)"
	# shellcheck disable=SC2064
	trap "rm -rf \"$TMPDIR_CB\"" EXIT INT TERM

	# Announce what we are about to do BEFORE installing.
	log "coffeebreak installer"
	log "  version: ${VERSION}"
	log "  target:  ${TARGET}"
	log "  source:  ${ASSET_URL}"
	log "  dest:    ${INSTALL_DIR}/${BIN_NAME}"

	# Download the asset and the checksums file.
	log "downloading asset..."
	download "$ASSET_URL" "${TMPDIR_CB}/${ASSET}"
	log "downloading SHA256SUMS..."
	download "$SUMS_URL" "${TMPDIR_CB}/SHA256SUMS"

	# Look up the expected checksum for our asset in SHA256SUMS.
	# Format per line: "<64-hex-sha256><two spaces><filename>".
	expected="$(awk -v f="$ASSET" '$2 == f { print $1 }' "${TMPDIR_CB}/SHA256SUMS")"
	[ -n "$expected" ] || err "no checksum entry for ${ASSET} in SHA256SUMS"

	# Compute the actual checksum and compare.
	actual="$(sha256_of "${TMPDIR_CB}/${ASSET}")"
	if [ "$expected" != "$actual" ]; then
		log "  expected: ${expected}"
		log "  actual:   ${actual}"
		err "checksum verification failed for ${ASSET}; aborting"
	fi
	log "checksum verified."

	# Extract the binary (it lives at the archive root, no nested dir).
	log "extracting..."
	tar -xzf "${TMPDIR_CB}/${ASSET}" -C "$TMPDIR_CB"
	[ -f "${TMPDIR_CB}/${BIN_NAME}" ] || err "archive did not contain a '${BIN_NAME}' binary at its root"

	# Install: create the dir, copy over (idempotent overwrite), make executable.
	mkdir -p "$INSTALL_DIR"
	# Use a temp name + mv for an atomic-ish replace (avoids "text file busy").
	cp "${TMPDIR_CB}/${BIN_NAME}" "${INSTALL_DIR}/${BIN_NAME}.new"
	chmod +x "${INSTALL_DIR}/${BIN_NAME}.new"
	mv -f "${INSTALL_DIR}/${BIN_NAME}.new" "${INSTALL_DIR}/${BIN_NAME}"

	# Report the installed version by invoking the binary.
	installed_version="$("${INSTALL_DIR}/${BIN_NAME}" --version 2>/dev/null || true)"
	if [ -n "$installed_version" ]; then
		log "installed: ${installed_version}"
	else
		log "installed coffeebreak to ${INSTALL_DIR}/${BIN_NAME}"
	fi

	# PATH hint: warn if the install dir is not already on $PATH.
	case ":${PATH}:" in
		*":${INSTALL_DIR}:"*)
			: # already on PATH, nothing to do
			;;
		*)
			# Pick a sensible rc file line based on the user's shell. These are
			# literal paths shown to the user (not expanded), so the `~` is fine.
			shell_name="$(basename "${SHELL:-sh}")"
			# shellcheck disable=SC2088
			case "$shell_name" in
				zsh) rc_file="~/.zshrc" ;;
				bash) rc_file="~/.bashrc" ;;
				fish) rc_file="~/.config/fish/config.fish" ;;
				*) rc_file="your shell's startup file" ;;
			esac

			log ""
			log "note: ${INSTALL_DIR} is not on your PATH."
			if [ "$shell_name" = "fish" ]; then
				log "      Add it by running:"
				log "        fish_add_path ${INSTALL_DIR}"
				log "      (or add 'set -gx PATH ${INSTALL_DIR} \$PATH' to ${rc_file})"
			else
				log "      Add this line to ${rc_file} and restart your shell:"
				log "        export PATH=\"${INSTALL_DIR}:\$PATH\""
			fi
			;;
	esac

	log ""
	log "Done. Get started with:"
	log "  coffeebreak           # 25 min focus / 5 min break"
	log "  coffeebreak --stats   # see your focus stats"
}

main "$@"
