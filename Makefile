libbad.a: bad.c
	clang -c -O3 -fPIE -o bad.o bad.c
	ar rcs libbad.a bad.o

.PHONY: clean
clean:
	rm -rf *.o
	rm -rf *.a
