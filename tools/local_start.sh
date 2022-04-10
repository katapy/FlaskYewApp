#!/bin/bash

cd ..

# ui derectory action.
(
    cd ui
    trunk build --release --public-url flask_yew_app
)
echo "ui build complete"

# server derectory action.
(
    cd server
    FLASK_APP=app/hello.py flask run
)
