cargo lambda invoke --remote \
  --data-ascii '{"name": "hello from rust"}' \
  --output-format json \
  rust-lambda-handshake