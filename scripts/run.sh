$arg = "$1"

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        echo "OS: $OSTYPE"
        
        if [$arg -eq "release"]; then
                ./target/release/rust_start
        elif [$arg -eq "debug"]; then
                ./target/debug/rust_start
        else
                echo "Unknown build type: $arg; building debug version..."
                ./target/debug/rust_start
        fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
        echo "OS: Mac OSX ($OSTYPE)"
        
        if [$arg -eq "release"]; then
                ./target/release/rust_start
        elif [$arg -eq "debug"]; then
                ./target/debug/rust_start
        else
                echo "Unknown build type: $arg; building debug version..."
                ./target/debug/rust_start
        fi
elif [[ "$OSTYPE" == "cygwin" ]]; then
        # POSIX compatibility layer and Linux environment emulation for Windows
        echo "OS: Cygwin ($OSTYPE)"
        
        if [$arg -eq "release"]; then
                ./target/release/rust_start.exe
        elif [$arg -eq "debug"]; then
                ./target/debug/rust_start.exe
        else
                echo "Unknown build type: $arg; building debug version..."
                ./target/debug/rust_start.exe
        fi
elif [[ "$OSTYPE" == "msys" ]]; then
        # Lightweight shell and GNU utilities compiled for Windows (part of MinGW)
        echo "OS: Windows-MinGW ($OSTYPE)"

        if [$arg -eq "release"]; then
                ./target/release/rust_start.exe
        elif [$arg -eq "debug"]; then
                ./target/debug/rust_start.exe
        else
                echo "Unknown build type: $arg; building debug version..."
                ./target/debug/rust_start.exe
        fi
else
        echo "Unknown OS type: $OSTYPE"
fi
