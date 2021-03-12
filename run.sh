#!/bin/bash

# cargo is a lot better for this
# but I'm using a VM without cargo to test this program
# so this script is meant to easily run dpkg for stagin
# and rs-dpkg to test the source code.

ROOT_PATH="/home/mhenderson/rs-dpkg"
DEBUG_PATH="${ROOT_PATH}/target/debug/"
RELEASE_PATH="${ROOT_PATH}/target/release/"
PROGRAM="rs-dpkg"
DEBUG_PROGRAM=${DEBUG_PATH}${PROGRAM}
RELEASE_PROGRAM=${RELEASE_PATH}${PROGRAM}

# Use dpkg for staging the environment for my copy
DEBIAN_FILE="tcsh_6.20.00-7+b1_amd64.deb"
PACKAGE_NAME="tcsh"
DPKG_INSTALL="dpkg -i ${DEBIAN_FILE}"
DPKG_REMOVE="dpkg --remove ${DEBIAN_FILE}"
DPKG_PURGE="dpkg --purge ${DEBIAN_FILE}"

function main()
{
    if [[ $# -eq 0 ]]
    then
        local opt="debug"
    else
        local opt=$1
    fi

    case $opt in
        debug)
            ${DEBUG_PROGRAM} $2
            ;;
        release)
            ${RELEASE_PROGRAM} $2
            ;;
        dpkg-install)
            dpkg --install ${DEBIAN_FILE}
            ;;
        dpkg-remove)
            dpkg --remove ${PACKAGE_NAME}
            ;;
        dpkg-purge)
            dpkg --purge ${PACKAGE_NAME}
            ;;
        *)
            echo "Invalid option: ${opt}"
            ;;

    esac
}

main $@