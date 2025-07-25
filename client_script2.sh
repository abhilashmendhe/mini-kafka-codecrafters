#!/bin/bash

# Create a named pipe
PIPE=$(mktemp -u)
mkfifo "$PIPE"

# Start nc in background: input from pipe, output to hexdump
nc localhost 9092 < "$PIPE" | hexdump -C &

# Send messages into the pipe
{
   echo -n "00000023001500046f7fc66100096b61666b612d636c69000a6b61666b612d636c6904302e3100" | xxd -r -p
   sleep 3
   echo -n "00000023001500046f7fc66100096b61666b612d636c69000a6b61666b612d636c6904302e3100" | xxd -r -p
} > "$PIPE"

# Clean up
sleep 2
rm "$PIPE"

