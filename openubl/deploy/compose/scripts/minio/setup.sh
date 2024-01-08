#!/bin/sh -l

# Connect to minio
/usr/bin/mc config host add myminio http://minio:9000 admin password;

# Create buckets
/usr/bin/mc mb myminio/ubl || true;
/usr/bin/mc mb myminio/index || true;

# Config events
/usr/bin/mc event add myminio/ubl arn:minio:sqs::UBL:nats --event "put,delete";

# Restart service
/usr/bin/mc admin service restart myminio;