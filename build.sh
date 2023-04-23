ARGS=$(getopt -o pbdha --long help,docker,Dtag,Dpush,build,pull,all,devEnv -- "$@")

if [ $? -ne 0 ]; then
    echo "you can use -h or --help for help"
    exit 1
fi

eval set -- "$ARGS"
while true
do
    case "$1" in
        -p|--pull)
            echo "git pull"
            git pull
            shift;;
        -b|--build)
            echo "build release"
            cargo build --release
            shift;;
        -d|--docker)
            echo "build for docker image"
            docker build -t selp-core:latest .
            docker tag selp-core:latest 49.232.216.15:8000/shengqian/selp-core:latest
            shift;;
        -t|--Dtag)
            echo "docker tag selp-core:$3"
            docker tag selp-core:latest 49.232.216.15:8000/shengqian/selp-core:"$3"
            shift;;
        --Dpush)
            echo "docker push selp-core:$3"
            docker push 49.232.216.15:8000/shengqian/selp-core:"$3"
            shift;;
        -a|--all)
            echo "build for docker image"
            docker build -t 49.232.216.15:8000/shengqian/selp-core:"$3" .
            echo "docker push selp-core:$3"
            docker push 49.232.216.15:8000/shengqian/selp-core:"$3"
            shift;;
        -h|--help|--)
            echo "-h or --help    for help"
            echo "-p or --pull    run git pull"
            echo "-b or --build   build release"
            echo "-d or --docker  build for docker image"
            echo "--Dtag    docker tag image"
            echo "--Dpush   docker push image"
            break;;
    esac
done
