CC       = clang++
CC_FLAGS = -Wall -Wpedantic -std=c++20
OPENBLAS = -lopenblas
OPT      = -O2
DEBUG    = -fsanitize=memory -g

run: debug
	./code
time: opt
	time ./code
gdb: debug
	gdb ./code
perf: opt
	perf record --call-graph=lbr ./code
	perf report --heirarchy -M intel
opt: clean
	${CC} ${CC_FLAGS} ${OPENBLAS} ${OPT} -o code code.cpp
debug: clean
	${CC} ${CC_FLAGS} ${OPENBLAS} ${DEBUG} -o code code.cpp
clean:
	rm -rf code
