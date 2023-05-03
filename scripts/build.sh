## Usage: sh scrpts/build.sh --arg=release

arg="$1"

if [[ "$arg" == "release" ]]; then
    echo "Building release version..."
    cargo build --release
    echo "...build complete."
elif [[ "$arg" == "debug" ]]; then
    echo "Building debug version..."
    cargo build
    echo "...build complete."
else
    echo "Unknown build type: $arg; building debug version..."
    cargo build
    echo "...build complete."
fi

