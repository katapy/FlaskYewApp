
import os
from flask import Flask
from flask import render_template
 
app = Flask(__name__)

@app.route("/")
def hello():
    return render_template('index.html')

@app.errorhandler(404)
def page_not_found(error):
    """If 404 error, go to yew 404 error page."""
    return render_template('index.html')

if __name__ == "__main__":
    app.run(debug=False, host='0.0.0.0', port=int(os.environ.get('PORT', 5000)))
