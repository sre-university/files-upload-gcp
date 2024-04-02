# !/usr/bin/env -S just --justfile
GOOGLE_PROJECT_ID:='cybx-chat'
IMAGE_LOCATION:='gcr.io'
IMAGE_NAME:='files-upload-gcp'
IMAGE_TAG:='v0.1.0'

_default:
  just --choose

all:
  just --list -u

docker-build:
	docker build --target runtime -t {{IMAGE_LOCATION}}/{{GOOGLE_PROJECT_ID}}/{{IMAGE_NAME}}:{{IMAGE_TAG}} -f Dockerfile .

docker-build-debian:
	docker build -t {{IMAGE_LOCATION}}/{{GOOGLE_PROJECT_ID}}/{{IMAGE_NAME}}:{{IMAGE_TAG}} -f debian/Dockerfile .

docker-push:
	docker push {{IMAGE_LOCATION}}/{{GOOGLE_PROJECT_ID}}/{{IMAGE_NAME}}:{{IMAGE_TAG}}

google-deploy:
  gcloud run deploy files-upload-gcp \
  --image={{IMAGE_LOCATION}}/{{GOOGLE_PROJECT_ID}}/{{IMAGE_NAME}}:{{IMAGE_TAG}} \
  --no-allow-unauthenticated \
  --port=8080 \
  --service-account=977645940426-compute@developer.gserviceaccount.com \
  --max-instances=1 \
  --set-env-vars='APPLICATION_URL=http://localhost' \
  --set-env-vars=APPLICATION_PORT=8080 \
  --set-env-vars=APPLICATION_BIND=0.0.0.0 \
  --set-env-vars=APPLICATION_WORKERS=5 \
  --set-env-vars=LOG_FORMAT=json \
  --set-env-vars=LOG_LEVEL=debug \
  --region=us-central1 \
  --project={{GOOGLE_PROJECT_ID}}

# google-deploy:
#   gcloud run deploy files-upload-gcp \
#   --image={{IMAGE_LOCATION}}/{{GOOGLE_PROJECT_ID}}/{{IMAGE_NAME}}:{{IMAGE_TAG}} \
#   --allow-unauthenticated \
#   --port=8080 \
#   --service-account=977645940426-compute@developer.gserviceaccount.com \
#   --max-instances=1 \
#   --set-env-vars='APPLICATION_URL=http://localhost' \
#   --set-env-vars=APPLICATION_PORT=8080 \
#   --set-env-vars=APPLICATION_BIND=0.0.0.0 \
#   --set-env-vars=APPLICATION_WORKERS=5 \
#   --set-env-vars=LOG_FORMAT=json \
#   --set-env-vars=LOG_LEVEL=debug \
#   --ingress=internal-and-cloud-load-balancing \
#   --region=us-central1 \
#   --project={{GOOGLE_PROJECT_ID}}

# docker push {{IMAGE_LOCATION}}/{{GOOGLE_PROJECT_ID}}/{{IMAGE_NAME}}:{{IMAGE_TAG}}

docker-run:
	docker run -p 8080:8080 {{IMAGE_LOCATION}}/{{GOOGLE_PROJECT_ID}}/{{IMAGE_NAME}}:{{IMAGE_TAG}}


# run build
example GOOGLE_PROJECT_ID=GOOGLE_PROJECT_ID:
  echo {{IMAGE_LOCATION}}/{{GOOGLE_PROJECT_ID}}/{{IMAGE_NAME}}:{{IMAGE_TAG}}

js:
  #!/usr/bin/env node
  console.log('Greetings from JavaScript!')

sh:
  #!/usr/bin/env sh
  hello='Yo'
  echo "$hello from a shell script!"

python:
  #!/usr/bin/env python3
  print('Hello from python!')

foo:
  #!/usr/bin/env bash
  set -euxo pipefail
  hello='Yo'
  echo "$hello from Bash!"