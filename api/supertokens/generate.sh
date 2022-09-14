docker run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate \
    -i https://app.swaggerhub.com/apiproxy/schema/file/apis/supertokens/CDI/2.15.1?format=json \
    -g rust \
    --additional-properties=packageName=supertokens \
    --additional-properties=library=hyper \
    -o /local
