#!/bin/bash
set -e

# Checking if UID and GID are passed as environment variables and need changing
current_uid=$(id -u user)
current_gid=$(id -g user)

if [ "$LOCAL_UID" != "$current_uid" ]; then
    echo "Updating UID from $current_uid to $LOCAL_UID"
    usermod -u $LOCAL_UID user
fi

if [ "$LOCAL_GID" != "$current_gid" ]; then
    echo "Updating GID from $current_gid to $LOCAL_GID"
    groupmod -g $LOCAL_GID user
fi

# Correct ownership if necessary (e.g., after ID changes)
chown -R user:user /home/user

# Execute the passed command
exec "$@"
