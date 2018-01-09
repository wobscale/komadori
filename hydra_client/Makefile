HYDRA_VERSION=v0.10.10

generate: ./hydra-$(HYDRA_VERSION).swagger.json
ifndef SWAGGER_CLI
	$(error "SWAGGER_CLI must be set to the path of swagger-codegen-cli.jar")
endif
	java -jar "$(SWAGGER_CLI)" generate -l rust -i ../api.swagger.json -DpackageName=hydra_client -DpackageVersion=0.1.0 -o .

./hydra-$(HYDRA_VERSION).swagger.json:
	curl -o ./hydra-$(HYDRA_VERSION).swagger.json https://raw.githubusercontent.com/ory/hydra/$(HYDRA_VERSION)/docs/api.swagger.json
