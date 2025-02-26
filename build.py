#!/usr/bin/env python3

import subprocess
import os
import argparse

import subprocess
import os
import sys
import threading
import argparse

def stream_output(pipe, func):
    """Helper function to stream output from subprocess pipes."""
    while True:
        line = pipe.readline()
        if not line:
            break
        func(line)

def run_docker_command(release=False):
    # Define the base command
    command = [
        "docker", "run", "--rm",
        "-e", f"LOCAL_UID={os.getuid()}",
        "-e", f"LOCAL_GID={os.getgid()}",
        "-v", f"{os.getcwd()}:/app",
        "ejortega/duo-rust:2.0", "cargo", "build",
        "--target", "riscv64gc-unknown-linux-musl",
    ]

    # Add the --release flag if needed
    if release:
        command.append("--release")

    # Start the process
    process = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, bufsize=1)

    # Create threads to handle stdout and stderr output
    stdout_thread = threading.Thread(target=stream_output, args=(process.stdout, sys.stdout.write))
    stderr_thread = threading.Thread(target=stream_output, args=(process.stderr, sys.stderr.write))

    # Start threads
    stdout_thread.start()
    stderr_thread.start()

    # Wait for process to complete
    process.wait()

    # Wait for all output to be printed
    stdout_thread.join()
    stderr_thread.join()

    # Print final exit status
    if process.returncode == 0:
        print("Command executed successfully")
    else:
        print("Command failed with return code:", process.returncode)

def main():
    parser = argparse.ArgumentParser(description="Run Docker commands for Rust builds")
    parser.add_argument('--release', action='store_true', help='Run the build in release mode')
    args = parser.parse_args()

    # Choose build type based on the command line argument
    if args.release:
        print("Running release build...")
        run_docker_command(release=True)
    else:
        print("Running debug build...")
        run_docker_command(release=False)

if __name__ == "__main__":
    main()
