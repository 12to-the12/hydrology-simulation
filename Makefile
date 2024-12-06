


default: run
all: install run

install:
	setup 3.12 uv

run:
	.venv/bin/python src/main.py

test:
	.venv/bin/python -m pytest

lint:
	.venv/bin/python -m mypy ./src/

clean:
	trash {./venv/,./.venv/}
