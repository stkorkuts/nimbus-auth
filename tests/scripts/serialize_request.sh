#!/bin/bash
# Usage: ./serialize_request.sh path/to/request.textproto [proto-file]
# Reads message type from request.meta in the same folder if available

TEXTPROTO_FILE="$1"
PROTO_FILE="$2"

if [ -z "$TEXTPROTO_FILE" ]; then
  echo "Usage: $0 <textproto-file> [proto-file]"
  exit 1
fi

DIR="$(dirname "$TEXTPROTO_FILE")"
META_FILE="$DIR/request.meta"

# Read message type from request.meta if it exists
if [ -f "$META_FILE" ]; then
  MESSAGE_TYPE=$(grep '^request_proto_definition=' "$META_FILE" | cut -d'=' -f2)
fi

if [ -z "$MESSAGE_TYPE" ]; then
  echo "Error: message type not found in request.meta"
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

OUTPUT_FILE="$DIR/$(basename "$TEXTPROTO_FILE" .textproto).bin"

# Include path for imports
INCLUDE_PATH="-I proto"

# Serialize textproto to protobuf binary
protoc $INCLUDE_PATH --encode="$MESSAGE_TYPE" "$PROTO_FILE" < "$TEXTPROTO_FILE" > "$OUTPUT_FILE"

echo "Protobuf binary written to $OUTPUT_FILE"
