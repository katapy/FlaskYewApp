
FROM python:alpine

WORKDIR /app

COPY ./app/ ./app/
COPY ./requirements.txt ./app/ 

RUN pip install -r app/requirements.txt

CMD ["python", "app/hello.py"]
