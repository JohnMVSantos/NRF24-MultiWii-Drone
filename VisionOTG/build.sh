#!/usr/bin/env bash
set -e

# Build VisionOTG Package on aarch64

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

if [ -f "$HOME/.cargo/env" ]; then
  source "$HOME/.cargo/env"
fi

DO_SETUP=false
for arg in "$@"; do
  case "$arg" in
    --setup)
      DO_SETUP=true
      ;;
    --help|-h)
      echo "Usage: $0 [--setup]"
      echo
      echo "  --setup    install dependencies, clone ONNX Runtime, and build the runtime"
      exit 0
      ;;
    *)
      echo "Unknown option: $arg"
      echo "Usage: $0 [--setup]"
      exit 1
      ;;
  esac
done

if [ "$DO_SETUP" = true ]; then
  # Update package list
  sudo apt update

  # Install Rust
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

  # Install GStreamer packages
  sudo apt install -y \
    libgstreamer1.0-dev \
    libgstreamer-plugins-base1.0-dev \
    gstreamer1.0-tools \
    gstreamer1.0-plugins-base \
    gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-bad \
    gstreamer1.0-libav \
    libglib2.0-dev \
    libcairo2-dev \
    pkg-config

  # Install Build and Python packages
  sudo apt install -y \
    build-essential \
    cmake \
    python3 \
    python3-pip \
    python3-venv \
    python3-full \
    git

  # Clone ONNX Runtime repository
  git clone --recursive https://github.com/microsoft/onnxruntime.git
  cd onnxruntime

  # Create build environment
  python3 -m venv "$PWD/cmake-env"
  source "$PWD/cmake-env/bin/activate"
  pip install cmake

  # Build ONNX Runtime package
  rm -rf ~/.cache/ort.pyke.io
  ./build.sh \
    --config Release \
    --build_shared_lib \
    --parallel 4 \
    --skip_tests

  # Alternative command if above fails
  # /home/pi/cmake-env/bin/cmake \
  #     --build build/Linux/Release \
  #     --target onnxruntime \
  #     -j2

  cd "$SCRIPT_DIR"
fi

# Build VisionOTG
export ORT_STRATEGY=system
export ORT_LIB_LOCATION="$SCRIPT_DIR/onnxruntime/build/Linux/Release"
export LD_LIBRARY_PATH="$SCRIPT_DIR/onnxruntime/build/Linux/Release:$LD_LIBRARY_PATH"

cargo build --release
