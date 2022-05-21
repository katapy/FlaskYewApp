
FROM python:alpine

WORKDIR /app

COPY ./app/ ./app/
COPY ./requirements.txt ./app/ 

# RUN pip install --upgrade pip
RUN pip install -r app/requirements.txt

CMD ["python", "app/hello.py"]
# CMD ["gunicorn", "-k", "flask_sockets.worker", "-b", "0.0.0.0:5000", "app.hello:app"]
