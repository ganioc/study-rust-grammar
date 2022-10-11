
CC=rustc


PROGS = hello condition reuse

all: $(PROGS)

%: %.rs 
	$(CC) $@.rs -o $@


clean:
	rm -f $(PROGS)