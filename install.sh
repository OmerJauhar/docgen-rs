#!/bin/bash

# DocGen Installation Script for Linux
# GreyBeard Outsourcing - Internal Tool
# Usage: ./install.sh [OPTIONS]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Default values
USER_INSTALL=false
INSTALL_PATH=""
SHOW_HELP=false

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --user)
            USER_INSTALL=true
            shift
            ;;
        --install-path)
            INSTALL_PATH="$2"
            shift 2
            ;;
        --help|-h)
            SHOW_HELP=true
            shift
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            exit 1
            ;;
    esac
done

# Show help
if [ "$SHOW_HELP" = true ]; then
    echo -e "${CYAN}DocGen Installation Script for Linux${NC}"
    echo "====================================="
    echo ""
    echo "Usage:"
    echo "  ./install.sh                    # Install system-wide (requires sudo)"
    echo "  ./install.sh --user             # Install for current user only"
    echo "  ./install.sh --install-path /opt/docgen  # Custom installation path"
    echo "  ./install.sh --help             # Show this help"
    echo ""
    exit 0
fi

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to check if running as root
is_root() {
    [ "$EUID" -eq 0 ]
}

# Main installation function
install_docgen() {
    echo -e "${GREEN}🚀 DocGen Installation for GreyBeard Outsourcing${NC}"
    echo "================================================"
    echo ""

    # Determine installation directory
    if [ -n "$INSTALL_PATH" ]; then
        INSTALL_DIR="$INSTALL_PATH"
    elif [ "$USER_INSTALL" = true ]; then
        INSTALL_DIR="$HOME/.local/bin"
    else
        if ! is_root; then
            echo -e "${RED}❌ System-wide installation requires sudo privileges.${NC}"
            echo -e "${YELLOW}   Please run with sudo or use --user flag.${NC}"
            exit 1
        fi
        INSTALL_DIR="/usr/local/bin"
    fi

    echo -e "${BLUE}📂 Installation directory: $INSTALL_DIR${NC}"

    # Create installation directory
    if [ ! -d "$INSTALL_DIR" ]; then
        mkdir -p "$INSTALL_DIR"
        echo -e "${GREEN}✅ Created installation directory${NC}"
    fi

    # Check dependencies
    echo -e "${BLUE}🔍 Checking dependencies...${NC}"

    # Check for Git
    if command_exists git; then
        GIT_VERSION=$(git --version)
        echo -e "${GREEN}✅ Git found: $GIT_VERSION${NC}"
    else
        echo -e "${RED}❌ Git not found. Installing Git...${NC}"
        
        # Detect package manager and install Git
        if command_exists apt-get; then
            sudo apt-get update && sudo apt-get install -y git
        elif command_exists yum; then
            sudo yum install -y git
        elif command_exists dnf; then
            sudo dnf install -y git
        elif command_exists pacman; then
            sudo pacman -S git
        else
            echo -e "${RED}❌ Could not detect package manager. Please install Git manually.${NC}"
            exit 1
        fi
        
        echo -e "${GREEN}✅ Git installed successfully${NC}"
    fi

    # Check for Rust
    if command_exists cargo; then
        RUST_VERSION=$(cargo --version)
        echo -e "${GREEN}✅ Rust found: $RUST_VERSION${NC}"
    else
        echo -e "${YELLOW}❌ Rust not found. Installing Rust...${NC}"
        
        # Install Rustup
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        
        # Source cargo environment
        source "$HOME/.cargo/env"
        
        echo -e "${GREEN}✅ Rust installed successfully${NC}"
    fi

    # Check for additional dependencies
    echo -e "${BLUE}🔍 Checking system dependencies...${NC}"
    
    MISSING_DEPS=()
    
    # Check for build essentials
    if ! command_exists gcc && ! command_exists clang; then
        MISSING_DEPS+=("build-essential")
    fi
    
    # Check for pkg-config
    if ! command_exists pkg-config; then
        MISSING_DEPS+=("pkg-config")
    fi
    
    # Check for OpenSSL development headers
    if [ ! -f "/usr/include/openssl/ssl.h" ] && [ ! -f "/usr/local/include/openssl/ssl.h" ]; then
        MISSING_DEPS+=("libssl-dev")
    fi

    # Install missing dependencies
    if [ ${#MISSING_DEPS[@]} -gt 0 ]; then
        echo -e "${YELLOW}📦 Installing system dependencies: ${MISSING_DEPS[*]}${NC}"
        
        if command_exists apt-get; then
            sudo apt-get update
            sudo apt-get install -y "${MISSING_DEPS[@]}"
        elif command_exists yum; then
            # Convert package names for RHEL/CentOS
            RHEL_DEPS=()
            for dep in "${MISSING_DEPS[@]}"; do
                case $dep in
                    "build-essential") RHEL_DEPS+=("gcc" "gcc-c+" "make") ;;
                    "libssl-dev") RHEL_DEPS+=("openssl-devel") ;;
                    *) RHEL_DEPS+=("$dep") ;;
                esac
            done
            sudo yum install -y "${RHEL_DEPS[@]}"
        elif command_exists dnf; then
            # Convert package names for Fedora
            FEDORA_DEPS=()
            for dep in "${MISSING_DEPS[@]}"; do
                case $dep in
                    "build-essential") FEDORA_DEPS+=("gcc" "gcc-c+" "make") ;;
                    "libssl-dev") FEDORA_DEPS+=("openssl-devel") ;;
                    *) FEDORA_DEPS+=("$dep") ;;
                esac
            done
            sudo dnf install -y "${FEDORA_DEPS[@]}"
        fi
        
        echo -e "${GREEN}✅ System dependencies installed${NC}"
    fi

    # Build DocGen
    echo -e "${BLUE}🔨 Building DocGen...${NC}"
    
    if [ -f "Cargo.toml" ]; then
        # Building from source
        cargo build --release
        SOURCE_BINARY="target/release/docgen"
    else
        echo -e "${RED}❌ Cargo.toml not found. Please run this script from the DocGen source directory.${NC}"
        exit 1
    fi

    # Copy binary to installation directory
    TARGET_BINARY="$INSTALL_DIR/docgen"
    cp "$SOURCE_BINARY" "$TARGET_BINARY"
    chmod +x "$TARGET_BINARY"
    echo -e "${GREEN}✅ Binary installed to $TARGET_BINARY${NC}"

    # Add to PATH if needed
    if [ "$USER_INSTALL" = true ]; then
        # Check if ~/.local/bin is in PATH
        if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
            echo -e "${BLUE}🔧 Adding to PATH...${NC}"
            
            # Add to shell profile
            SHELL_PROFILE=""
            if [ -f "$HOME/.bashrc" ]; then
                SHELL_PROFILE="$HOME/.bashrc"
            elif [ -f "$HOME/.zshrc" ]; then
                SHELL_PROFILE="$HOME/.zshrc"
            elif [ -f "$HOME/.profile" ]; then
                SHELL_PROFILE="$HOME/.profile"
            fi
            
            if [ -n "$SHELL_PROFILE" ]; then
                echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$SHELL_PROFILE"
                echo -e "${GREEN}✅ Added to PATH in $SHELL_PROFILE${NC}"
                echo -e "${YELLOW}   Please restart your terminal or run: source $SHELL_PROFILE${NC}"
            fi
        fi
    fi

    # Create man page (optional)
    if command_exists man; then
        MAN_DIR="/usr/local/share/man/man1"
        if [ "$USER_INSTALL" = true ]; then
            MAN_DIR="$HOME/.local/share/man/man1"
        fi
        
        if [ -w "$(dirname "$MAN_DIR")" ] || [ "$USER_INSTALL" = true ]; then
            mkdir -p "$MAN_DIR"
            cat > "$MAN_DIR/docgen.1" << 'EOF'
.TH DOCGEN 1 "2024" "1.0.0" "GreyBeard Outsourcing"
.SH NAME
docgen \- AI-powered documentation generator
.SH SYNOPSIS
.B docgen
[\fIcommand\fR] [\fIoptions\fR]
.SH DESCRIPTION
DocGen is an AI-powered documentation generator for software engineers at GreyBeard Outsourcing.
It analyzes git diffs and generates comprehensive documentation using AI.
.SH COMMANDS
.TP
.B generate
Launch the interactive documentation generator
.TP
.B config
Configure user information
.TP
.B version
Show version information
.SH AUTHOR
Omer Jauhar <omer.jauhar@greybeardsupport.com>
.SH SEE ALSO
git(1)
EOF
            echo -e "${GREEN}✅ Man page installed${NC}"
        fi
    fi

    echo ""
    echo -e "${GREEN}🎉 Installation completed successfully!${NC}"
    echo "================================================"
    echo "You can now run DocGen using any of these commands:"
    echo "  docgen generate              # Start the documentation generator"
    echo "  docgen config               # Configure user settings"
    echo "  docgen version              # Show version information"
    echo ""
    echo "For support, contact: omer.jauhar@greybeardsupport.com"
    echo ""
    
    # Test installation
    if command_exists docgen; then
        echo -e "${GREEN}✅ Installation verified - DocGen is ready to use!${NC}"
    else
        echo -e "${YELLOW}⚠️  DocGen installed but not in PATH. You may need to restart your terminal.${NC}"
    fi
}

# Run installation
install_docgen
