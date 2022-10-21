
DIRS= prj1 prj2


# for i in $P{DIRS}; do \
# 	(cd $i && echo "making $i" && $(MAKE)) || exit 1;
# done
# for i in ${DIRS}; do \
# 	echo "Go. "  \
# done

all:
	@for i in $(DIRS) ; do \
	echo $$i ; \
	(cd $$i && $(MAKE)) || exit 1;  \
	done

clean:
	@for i in $(DIRS); do \
	(cd $$i && echo "cleaning $$i" && $(MAKE) clean) || exit 1; \
	done



