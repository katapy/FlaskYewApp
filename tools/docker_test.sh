#!/bin/bash

cd ..
CONTAINER_NAME="flask-yew-app"

# ui derectory action.
(
    cd ui
    trunk build --release --public-url flask_yew_app
)
echo "ui build complete"

# Remove old file
(
    cd server/app
    # rm -rf static
)

rm -rf server/app/static
mkdir server/app/static
cp -r ui/dist server/app/static
(
    cd server/app/static
    JS_FILENAME=`ls -R1 | grep .js`
    WAMS_FILENAME=`ls -R1 | grep .wasm`
    echo "js name: $JS_FILENAME"
    echo "wasm filename: $WAMS_FILENAME"
    mkdir js
    (
        cd js
        echo "import init from \"/static/dist/$JS_FILENAME\";" > index.js
        echo "await init(\"/static/dist/$WAMS_FILENAME\");" >> index.js
    )
)



# server derectory action.
(
    cd server
    docker build -t $CONTAINER_NAME .
    docker run -p 5000:5000 -it $CONTAINER_NAME

    # Delete continer & image after test
    docker rm -f `docker ps -a | grep $CONTAINER_NAME | awk '{print $1}'`
    docker rmi -f `docker images | grep $CONTAINER_NAME | awk '{print $1}'`
)
