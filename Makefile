SHELL=/bin/bash

default:
    :

setup:
	pipenv sync

svg:
	@if [ "$(FIXTURE_DIR)" = "" ]; then echo "FIXTURE_DIR is not defined. Aborting."; exit 1; fi
	set -o pipefail; cat $(FIXTURE_DIR)/* | pipenv run ./main.py | dot -Tsvg > output.svg
	@echo "Created: output.svg"