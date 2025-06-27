


default: run
all: install run

install:
	setup 3.12 uv

run:
	- trash /home/logan/.cache/nim/sim_*/
	.venv/bin/python src/main.py

test:
	.venv/bin/python -m pytest ./src/*

lint:
	.venv/bin/python -m mypy ./src/

clean:
	- trash {./venv/,./.venv/}

grab:
	- rm ./remote/*.png
	rsync -avzPh services:~/projects/sim/{heightmap,delta,mag,paths,trace,x,y}.png \
	./remote/
	nemo ./remote/
