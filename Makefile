SHELL=/bin/bash

default:
    :

setup:
	pipenv sync

svg:
	@if [ "$(FIXTURE_DIR)" = "" ]; then echo "FIXTURE_DIR is not defined. Aborting."; exit 1; fi
	set -o pipefail; pipenv run ./main.py $(FIXTURE_DIR)/* | dot -Tsvg > output.svg
	@echo "Created: output.svg"