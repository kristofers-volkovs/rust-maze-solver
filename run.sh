#!/bin/bash

function show_usage (){
    printf "Usage: $0 [options [parameters]]\n"
    printf "\n"
    printf "Options:\n"
    printf " -r|--run - build and runs the project (default)"
    printf " -b|--build - only builds the project without running it"
    printf " -f|--fmt - formats the rust library"

    return 0
}

run="run"
build="build"
fmt="fmt"

cmd=$run

while [ ! -z "$1" ]; do
    case "$1" in
        -r|--run)
            ;;
        -b|--build)
            cmd=$build
            ;;
        -f|--fmt)
            cmd=$fmt
            ;;
        -h|--help)
            show_usage
            exit
            ;;
        *)
            printf "Wrong argument/s: '$1'\n\n"
            show_usage
            exit
            ;;
    esac
shift
done

lib_dir="lib/maze"
if [ $cmd == $run ]; then
    cd $lib_dir
    maturin develop

    cd "../.."
    python "src/main.py"

elif [ $cmd == $build ]; then
    cd $lib_dir
    maturin develop

    printf "\Project libraries built\n"

elif [ $cmd == $fmt ]; then
    cd $lib_dir
    cargo fmt

    printf "\nDir '$lib_dir' formated\n"

fi

