#!/bin/bash
tput setaf 44
tput bold
echo "🚧 Build now? ❓"
while true; do
    tput sgr0
    echo -n "Hit enter to build. Enter 'd' to be able to set breakpoints: "
    read INPUT
    tput setaf 178
    tput bold
    echo "🚧 Building..."
    ./build-release.sh
    if [[ $? == 0 ]]; then

        tput setaf 40
        tput bold
        echo -e "\n🚧 Build successful! ✔️"
        tput setaf 33
        echo "🔦 Flashing..."
        tput sgr0
        if [[ $INPUT = d* ]]; then
            ./gdb-release.sh
        else 
            echo -e "c\nq\n" | ./gdb-release.sh
        fi
        tput setaf 248
        tput bold
        echo -e "\n🚀 Run finished."
    else
        tput setaf 9
        tput bold
        echo -e "\n🚧 Build failed! ❌"
    fi
    echo -e "\n"
    tput setaf 44
    tput bold
    echo "🚧 Build again? ❓"       
done

