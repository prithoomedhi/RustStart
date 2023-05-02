# $_type = "$arg:-release}"

echo "arg = $arg"

if [$arg=="release"]
then
    echo "Building release version..."
    cargo build --release
    echo "...build complete."
elif [$arg=="debug"]
then
    echo "Building debug version..."
    cargo build --debug
    echo "...build complete."
fi