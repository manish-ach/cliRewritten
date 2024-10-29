#!/bin/bash

# Get the directory of the script
SCRIPT_DIR="$(dirname "$0")"  # This gets the directory of the current script

# Define the project directory as the current directory
PROJECT_DIR="$SCRIPT_DIR"

# Build the project in release mode
echo "Building the project at $PROJECT_DIR..."
cargo build --release --manifest-path "$PROJECT_DIR/cat/Cargo.toml"
cargo build --release --manifest-path "$PROJECT_DIR/echo/Cargo.toml"
cargo build --release --manifest-path "$PROJECT_DIR/ls/Cargo.toml"
cargo build --release --manifest-path "$PROJECT_DIR/find/Cargo.toml"

# Check if the build was successful
if [ $? -ne 0 ]; then
    echo "Cargo build failed. Exiting."
    exit 1
fi

# Define paths for the executable, destination, and symlink
LS_EXECUTABLE="$PROJECT_DIR/ls/target/release/ls"
LS_DESTINATION="/usr/local/bin/ls_rust"
LS_SYMLINK="/usr/local/bin/ruls"

CAT_EXECUTABLE="$PROJECT_DIR/cat/target/release/cat"
CAT_DESTINATION="/usr/local/bin/cat_rust"
CAT_SYMLINK="/usr/local/bin/rcat"

ECHO_EXECUTABLE="$PROJECT_DIR/echo/target/release/echo"
ECHO_DESTINATION="/usr/local/bin/echo_rust"
ECHO_SYMLINK="/usr/local/bin/recho"

FIND_EXECUTABLE="$PROJECT_DIR/find/target/release/find"
FIND_DESTINATION="/usr/local/bin/find_rust"
FIND_SYMLINK="/usr/local/bin/rfind"

# Check for root permissions
if [ "$EUID" -ne 0 ]; then
    echo "Please run as root (sudo)"
    exit
fi

# Move the built executable to /usr/local/bin/
echo "Moving executable to $LS_DESTINATION..."
sudo cp "$LS_EXECUTABLE" "$LS_DESTINATION"
echo "Moving executable to $CAT_DESTINATION..."
sudo cp "$CAT_EXECUTABLE" "$CAT_DESTINATION"
echo "Moving executable to $ECHO_DESTINATION..."
sudo cp "$ECHO_EXECUTABLE" "$ECHO_DESTINATION"
echo "Moving executable to $FIND_DESTINATION..."
sudo cp "$FIND_EXECUTABLE" "$FIND_DESTINATION"

# Check if the copy was successful
if [ $? -ne 0 ]; then
    echo "Failed to copy the executable to /usr/local/bin/. Exiting."
    exit 1
fi

# Create a symlink
echo "Creating symlink at $LS_SYMLINK..."
sudo ln -sf "$LS_DESTINATION" "$LS_SYMLINK"
echo "Installation complete! You can now run the program with 'ruls'"
echo "Creating symlink at $CAT_SYMLINK..."
sudo ln -sf "$CAT_DESTINATION" "$CAT_SYMLINK"
echo "Installation complete! You can now run the program with 'rcat'"
echo "Creating symlink at $ECHO_SYMLINK..."
sudo ln -sf "$ECHO_DESTINATION" "$ECHO_SYMLINK"
echo "Installation complete! You can now run the program with 'recho'"
echo "Creating symlink at $FIND_SYMLINK..."
sudo ln -sf "$FIND_DESTINATION" "$FIND_SYMLINK"
echo "Installation complete! You can now run the program with 'rfind'"
