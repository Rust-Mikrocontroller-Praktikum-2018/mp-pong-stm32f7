#!/bin/bash
tput setaf 44
tput bold
echo "🚧 Build now❓"
while true; do

    tput sgr0
    read
    tput setaf 178
    tput bold
    echo "🚧 Building..."
    ./build-release.sh
    if [[ $? == 0 ]]; then
        tput setaf 40
        tput bold
        echo "✔️  Build successful!"
        tput setaf 33
        echo "🚀 Running..."
        tput sgr0
        echo -e "c\nq\n" | ./gdb-release.sh
        echo "🚀 Run finished!"
    else
        tput setaf 9
        tput bold
        echo "❌ Build failed!"
        sleep 1
    fi
    echo -e "\n\n"
    tput setaf 44
    tput bold
    echo "🚧 Build again❓"       
done

