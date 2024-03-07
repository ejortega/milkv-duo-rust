#!/bin/bash
set -e

# Add local user
# Either use the LOCAL_UID and LOCAL_GID if passed in at runtime or
# fallback

USER_ID=${LOCAL_UID:-9001}
GROUP_ID=${LOCAL_GID:-9001}

echo "Starting with UID: $USER_ID, GID: $GROUP_ID"
groupadd -g $GROUP_ID user
useradd --shell /bin/bash -u $USER_ID -g $GROUP_ID -o -c "" -m user

export HOME=/home/user

# Use gosu (similar to su-exec) to switch to the new user
exec gosu user "$@"
