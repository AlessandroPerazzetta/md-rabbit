#!/usr/bin/env bash
set -e

# Script to install dependencies (detect os and use the right package manager) and build the project
# Dependencies: 
#   - libgtk-3-dev
#   - libglib2.0-dev

# Function to install dependencies on Debian-based systems
install_debian_dependencies() {
    echo "Installing dependencies for Debian-based system..."
    sudo apt-get update
    sudo apt-get install -y libgtk-3-dev libglib2.0-dev
}
# Function to install dependencies on Red Hat-based systems
install_redhat_dependencies() {
    echo "Installing dependencies for Red Hat-based system..."
    sudo yum install -y gtk3-devel glib2-devel
}
# Function to install dependencies on Arch-based systems
install_arch_dependencies() {
    echo "Installing dependencies for Arch-based system..."
    sudo pacman -Syu --noconfirm gtk3 glib2
}
# Detect OS and install dependencies
if [ -f /etc/debian_version ]; then
    install_debian_dependencies
elif [ -f /etc/redhat-release ]; then
    install_redhat_dependencies
elif [ -f /etc/arch-release ]; then
    install_arch_dependencies
else
    echo "Unsupported OS. Please install dependencies manually."
    exit 1
fi