#!/bin/bash
# Usage: ./deserialize_response.sh path/to/response.bin [proto-file]
# Reads message type from response.meta in the same folder if available

BIN_FILE="$1"
PROTO_FILE="$2"

if [ -z "$BIN_FILE" ]; then
  echo "Usage: $0 <bin-file> [proto-file]"
  exit 1
fi

DIR="$(dirname "$BIN_FILE")"
META_FILE="$DIR/response.meta"

# Read message type from request.meta if it exists
if [ -f "$META_FILE" ]; then
  MESSAGE_TYPE=$(grep '^response_proto_definition=' "$META_FILE" | cut -d'=' -f2)
fi

if [ -z "$MESSAGE_TYPE" ]; then
  echo "Error: message type not found in response.meta"
  exit 1
fi

# Use proto file if provided, otherwise default
if [ -z "$PROTO_FILE" ]; then
  PROTO_FILE="proto/v1/auth/signup.proto"
fi

if [ ! -f "$PROTO_FILE" ]; then
  echo "Error: proto file not found: $PROTO_FILE"
  exit 1
fi

OUTPUT_FILE="$DIR/$(basename "$BIN_FILE" .bin).textproto"

INCLUDE_PATH="-I proto"

# Deserialize binary to textproto
protoc $INCLUDE_PATH --decode="$MESSAGE_TYPE" "$PROTO_FILE" < "$BIN_FILE" > "$OUTPUT_FILE"

echo "Textproto written to $OUTPUT_FILE"
