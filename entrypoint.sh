#!/bin/bash
set -e

# Checking if UID and GID are passed as environment variables and need changing
current_uid=$(id -u user)
current_gid=$(id -g user)

if [ "$$" -ne 1 ]; then
    if [ "$LOCAL_UID" != "$current_uid" ]; then
        usermod -u $LOCAL_UID user
    fi

    if [ "$LOCAL_GID" != "$current_gid" ]; then
        groupmod -g $LOCAL_GID user
    fi
fi

# Execute the passed command
exec "$@"
