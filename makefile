


default: run
all: install run

install:
	setup 3.12 uv

debug:
	- trash ./src/__pycache__
	- trash /home/logan/.cache/nim/sim_*/
	nim compile --run --threads ./src/sim.nim
runpy:
	- trash ./src/__pycache__
	- trash /home/logan/.cache/nim/sim_*/
	.venv/bin/python pysrc/main.py
run:
	- trash ./src/__pycache__
	- trash /home/logan/.cache/nim/sim_*/
	nim compile --run --threads -d:release ./src/sim.nim

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
