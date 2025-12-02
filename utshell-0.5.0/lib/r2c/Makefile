all:libr2c.a


CC = gcc
AR = ar

CFLAGS = -c -g -fPIC
AFLAGS = -r

OBJS = r2c.o

SOURS = r2c.c

LIBNAME = libr2c.a

libr2c.a:$(OBJS)
	$(AR) $(AFLAGS) libr2c.a $(OBJS)

$(OBJS):$(SOURS)
	$(CC) $(CFLAGS) $< -o $@

r2c.o:r2c.c

.PHONY:clean
clean:
	rm -rf *.o *.a
