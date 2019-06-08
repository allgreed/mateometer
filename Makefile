help:
	echo "ble"

docker:
	docker build -t allgreed/mateometer:preview1 .

docker-deploy:
	docker push allgreed/mateometer:preview1
