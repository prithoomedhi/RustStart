if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        echo "OS: $OSTYPE"
        ./target/release/rust_start
elif [[ "$OSTYPE" == "darwin"* ]]; then
        echo "OS: Mac OSX ($OSTYPE)"
        ./target/release/rust_start
elif [[ "$OSTYPE" == "cygwin" ]]; then
        # POSIX compatibility layer and Linux environment emulation for Windows
        echo "OS: Cygwin ($OSTYPE)"
        ./target/release/rust_start.exe
elif [[ "$OSTYPE" == "msys" ]]; then
        # Lightweight shell and GNU utilities compiled for Windows (part of MinGW)
        echo "OS: Windows-MinGW ($OSTYPE)"
        ./target/release/rust_start.exe
else
        echo "Unknown OS type: $OSTYPE"
fi
