#!/bin/sh
# coffeebreak uninstaller (Unix)
#
# Removes the coffeebreak binary plus its config and data directories.
#
# Usage (one-liner):
#   curl -fsSL https://raw.githubusercontent.com/leuchtturm/coffeebreak/main/uninstall.sh | sh
#
# Environment overrides:
#   COFFEEBREAK_INSTALL_DIR   directory the binary lives in (default: $HOME/.local/bin)
#   COFFEEBREAK_YES=1         skip the confirmation prompt (required when no TTY is available)
#
# This script is intentionally conservative: it only ever touches a fixed
# set of well-known paths and refuses to remove anything dangerous.

set -eu

# --- configuration ----------------------------------------------------------

# Primary install dir (overridable), matching install.sh.
INSTALL_DIR="${COFFEEBREAK_INSTALL_DIR:-$HOME/.local/bin}"

# Binary name.
BIN_NAME="coffeebreak"

# Config and data directories created by the app at runtime. The config dir
# mirrors the Rust app (paths.rs): $XDG_CONFIG_HOME/coffeebreak when set to an
# absolute path, otherwise ~/.config/coffeebreak.
if [ -n "${XDG_CONFIG_HOME:-}" ] && [ "${XDG_CONFIG_HOME#/}" != "$XDG_CONFIG_HOME" ]; then
	CONFIG_DIR="$XDG_CONFIG_HOME/coffeebreak"
else
	CONFIG_DIR="$HOME/.config/coffeebreak"
fi
DATA_DIR="$HOME/.coffeebreak"

# --- helpers ----------------------------------------------------------------

# Print an error message to stderr and exit.
die() {
	printf 'error: %s\n' "$1" >&2
	exit 1
}

# Guard against removing dangerous / unintended paths.
#
# Rejects: empty/unset, literal "/", and exactly $HOME. This protects against
# an unset $HOME (which would otherwise collapse "$HOME/.coffeebreak" to
# "/.coffeebreak", or "$HOME" to "") or other surprises.
is_safe_path() {
	_p="$1"
	# Empty or unset.
	[ -n "$_p" ] || return 1
	# Filesystem root.
	[ "$_p" != "/" ] || return 1
	# $HOME itself (only meaningful when HOME is set and non-empty).
	if [ -n "${HOME:-}" ] && [ "$_p" = "$HOME" ]; then
		return 1
	fi
	return 0
}

# Remove a single file (or symlink) if it exists, after a safety check.
remove_file() {
	_target="$1"
	# Only act on a path that is safe AND exists as a file or symlink. We never
	# follow the link target -- rm -f removes the link/file at this exact path.
	if is_safe_path "$_target" && { [ -e "$_target" ] || [ -L "$_target" ]; }; then
		rm -f -- "$_target"
		printf '  removed %s\n' "$_target"
		REMOVED_ANY=1
	fi
}

# Remove a directory tree if it exists, after a safety check.
remove_dir() {
	_target="$1"
	if is_safe_path "$_target" && [ -d "$_target" ]; then
		rm -rf -- "$_target"
		printf '  removed %s\n' "$_target"
		REMOVED_ANY=1
	fi
}

# --- discover what exists ---------------------------------------------------

# Candidate binary locations. The primary install dir comes first; then a
# couple of common alternates are checked but only acted upon if present.
BIN_PRIMARY="$INSTALL_DIR/$BIN_NAME"
BIN_USR_LOCAL="/usr/local/bin/$BIN_NAME"
BIN_CARGO="$HOME/.cargo/bin/$BIN_NAME"

# Build the list of paths that actually exist and are safe to remove.
# We collect them so we can show the user before doing anything.
EXISTING=""

add_existing() {
	# $1 is the path; only add if it exists (file/symlink) or is a dir.
	if is_safe_path "$1" && { [ -e "$1" ] || [ -L "$1" ]; }; then
		# Avoid duplicates (e.g. INSTALL_DIR == /usr/local/bin).
		case " $EXISTING " in
		*" $1 "*) : ;;
		*) EXISTING="$EXISTING$1
" ;;
		esac
	fi
}

add_existing "$BIN_PRIMARY"
add_existing "$BIN_USR_LOCAL"
add_existing "$BIN_CARGO"
add_existing "$CONFIG_DIR"
add_existing "$DATA_DIR"

# Nothing to do?
if [ -z "$EXISTING" ]; then
	printf 'coffeebreak does not appear to be installed; nothing to remove.\n'
	printf '(checked: %s, %s, %s, %s, %s)\n' \
		"$BIN_PRIMARY" "$BIN_USR_LOCAL" "$BIN_CARGO" "$CONFIG_DIR" "$DATA_DIR"
	exit 0
fi

# --- show plan & confirm ----------------------------------------------------

printf 'The following will be removed:\n\n'
# Print each discovered path indented.
printf '%s' "$EXISTING" | while IFS= read -r _line; do
	[ -n "$_line" ] && printf '  %s\n' "$_line"
done
printf '\n'

confirm() {
	# Non-interactive override.
	if [ "${COFFEEBREAK_YES:-}" = "1" ]; then
		return 0
	fi

	# Try to read confirmation from the controlling terminal. This is required
	# under `curl ... | sh` because stdin is the piped script, not the user.
	if [ -r /dev/tty ]; then
		printf 'Proceed with uninstall? [y/N] '
		# Read a single line from the TTY.
		if IFS= read -r _ans </dev/tty; then
			case "$_ans" in
			y | Y | yes | YES | Yes) return 0 ;;
			*) return 1 ;;
			esac
		fi
		# read failed (EOF on tty): treat as decline.
		return 1
	fi

	# No TTY and no COFFEEBREAK_YES=1: refuse to act, to stay safe.
	die "no terminal available for confirmation; re-run with COFFEEBREAK_YES=1 to proceed non-interactively"
}

if ! confirm; then
	printf 'Aborted; nothing was removed.\n'
	exit 0
fi

# --- perform removal --------------------------------------------------------

REMOVED_ANY=0

printf '\nRemoving coffeebreak...\n'

# Binaries (files/symlinks).
remove_file "$BIN_PRIMARY"
remove_file "$BIN_USR_LOCAL"
remove_file "$BIN_CARGO"

# Config and data directories (trees).
remove_dir "$CONFIG_DIR"
remove_dir "$DATA_DIR"

# --- done -------------------------------------------------------------------

if [ "$REMOVED_ANY" = "1" ]; then
	printf '\ncoffeebreak has been uninstalled. Thanks for taking breaks -- see you next time!\n'
else
	printf '\nNothing was removed.\n'
fi
