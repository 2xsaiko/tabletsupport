#!/bin/bash

DEV_NAME='ELAN0732:00 04F3:2650'

case "$1" in
'normal')
    mat=(
        1 0 0
        0 1 0
        0 0 1
    )
    ;;
'left-up')
    mat=(
        0 -1 1
        1 0 0
        0 0 1
    )
    ;;
'right-up')
    mat=(
        0 1 0
        -1 0 1
        0 0 1
    )
    ;;
'bottom-up')
    mat=(
        -1 0 1
        0 -1 1
        0 0 1
    )
    ;;
*)
    echo "Invalid rotation: $1"
    exit 1
esac

xinput set-prop "${DEV_NAME}" 'Coordinate Transformation Matrix' "${mat[@]}"
