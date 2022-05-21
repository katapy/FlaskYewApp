
# -*- coding: utf-8 -*-
import os
from flask import Flask, render_template
from flask_sock import Sock

app = Flask(__name__)
sock = Sock(app)

@app.route("/")
def hello():
    return render_template('index.html')

@app.route("/ws")
def ws_test():
    return render_template('ws_test.html')


@app.errorhandler(404)
def page_not_found(error):
    """If 404 error, go to yew 404 error page."""
    return render_template('index.html')

@sock.route('/ws')
def echo(ws):
    while True:
        data = ws.receive()
        ws.send("ws server send: " + data)

if __name__ == "__main__":
    app.run(debug=False, host='0.0.0.0', port=int(os.environ.get('PORT', 5000)))
