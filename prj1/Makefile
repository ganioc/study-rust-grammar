
CC=rustc


PROGS = hello condition reuse codeError matching owner

all: $(PROGS)

%: %.rs 
	$(CC) $@.rs -o $@


clean:
	rm -f $(PROGS)