
SOURCES=src/movie_hash.c
OBJS=movie_hash.o
LIB=libmovie_hash.a

libmovie_hash:
	clang -c $(SOURCES) -o $(OBJS) -O3 -Werror -Weverything -Wno-padded
	ar rcvs $(LIB) $(OBJS)

all: libmovie_hash
	cargo build
