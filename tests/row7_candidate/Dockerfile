ARG BASE_IMAGE
FROM ${BASE_IMAGE}
RUN apt-get update \
 && apt-get install -y --no-install-recommends gcc libc6-dev ca-certificates \
 && rm -rf /var/lib/apt/lists/*
RUN for command in python python3 node nodejs; do \
      if command -v "$command"; then exit 1; fi; \
    done
