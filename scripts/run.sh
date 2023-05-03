arg="$1"

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        echo "OS: $OSTYPE"
        
        if [[ "$arg" == "release" ]]; then
                ./target/release/rust_start
        elif [[ "$arg" == "debug" ]]; then
                ./target/debug/rust_start
        else
                if [[ "$arg" == "" ]]; then
                        arg="NULL"
                elif [ -z "$arg" ]; then
                        arg="NULL"
                fi
                echo "Unknown build type:    $arg"
        fi
        
elif [[ "$OSTYPE" == "darwin"* ]]; then
        echo "OS: Mac OSX ($OSTYPE)"
        
        if [[ "$arg" == "release" ]]; then
                ./target/release/rust_start
        elif [[ "$arg" == "debug" ]]; then
                ./target/debug/rust_start
        else
                if [[ "$arg" == "" ]]; then
                        arg="NULL"
                elif [ -z "$arg" ]; then
                        arg="NULL"
                fi
                echo "Unknown build type:    $arg"
        fi
        
elif [[ "$OSTYPE" == "cygwin" ]]; then
        # POSIX compatibility layer and Linux environment emulation for Windows
        echo "OS: Cygwin ($OSTYPE)"
        
        if [[ "$arg" == "release" ]]; then
                ./target/release/rust_start.exe
        elif [[ "$arg" == "debug" ]]; then
                ./target/debug/rust_start.exe
        else
                if [[ "$arg" == "" ]]; then
                        arg="NULL"
                elif [ -z "$arg" ]; then
                        arg="NULL"
                fi
                echo "Unknown build type:    $arg"
        fi

elif [[ "$OSTYPE" == "msys" ]]; then
        # Lightweight shell and GNU utilities compiled for Windows (part of MinGW)
        echo "OS: Windows-MinGW ($OSTYPE)"

        if [[ "$arg" == "release" ]]; then
                ./target/release/rust_start.exe
        elif [[ "$arg" == "debug" ]]; then
                ./target/debug/rust_start.exe
        else
                if [[ "$arg" == "" ]]; then
                        arg="NULL"
                elif [ -z "$arg" ]; then
                        arg="NULL"
                fi
                echo "Unknown build type:    $arg"
        fi
else
        echo "Unknown OS type: $OSTYPE"
fi
