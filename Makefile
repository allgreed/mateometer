.DEFAULT_GOAL := help

.PHONY: docker docker-publish
docker: ## build Docker image
	docker build -t allgreed/mateometer:preview2 -f deploy/Dockerfile .

docker-publish: ## push Docker image
	docker push allgreed/mateometer:preview2


.PHONY: help
help: ## print this message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
