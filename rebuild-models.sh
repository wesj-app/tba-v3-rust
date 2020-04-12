#!/usr/bin/env sh

OPENAPI_SPEC="https://www.thebluealliance.com/swagger/api_v3.json"
MODELS_DIR="lib-models"

rm -rf $MODELS_DIR
openapi-generator generate -i $OPENAPI_SPEC -Dmodels -c models-config.yml -g rust -o $MODELS_DIR
patch -d $MODELS_DIR -p0 < models.patch