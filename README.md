# Flask Yew App

## Pull docker image

```bash
docker pull ghcr.io/katapy/flask-yew-app:develop
docker run -p 5000:5000 -it ghcr.io/katapy/flask-yew-app:develop
```

see detail  
<https://github.com/users/katapy/packages/container/package/flask-yew-app>

## push Heroku

```bash
docker pull ghcr.io/katapy/flask-yew-app:master
heroku container:push web -a flask-yew-app
heroku container:release web -a flask-yew-app
```
