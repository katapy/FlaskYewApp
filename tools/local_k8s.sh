
APP_NAME=flask-yew-app

cd ..

(
    cd k8s
    minikube start

    # Run k8s app
    kubectl create namespace $APP_NAME
    kubectl apply -f deployment.yaml --namespace=$APP_NAME
    kubectl expose deployment $APP_NAME --type=NodePort --port=5000 --namespace=$APP_NAME
    
    # Open brower
    echo "Please wait 20 minutes"
    sleep 20
    URL=`minikube service $APP_NAME --url --namespace=$APP_NAME`
    open $URL

    # Wait for input
    read -p "Input if you want to end."
    # Delete
    kubectl delete service $APP_NAME --namespace=$APP_NAME
    kubectl delete deployment $APP_NAME --namespace=$APP_NAME
    kubectl delete namespace $APP_NAME
    # minikube stop
)
