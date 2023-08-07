#!/bin/bash

run:
	docker network create f-network || true
	docker-compose up -d

stop:
	docker-compose stop

down:
	docker-compose down

restart:
	$(MAKE) stop && $(MAKE) run

build:
	docker-compose build
