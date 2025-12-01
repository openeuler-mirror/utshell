Machine = $(shell uname -m)
MACHTYPE = $(shell ../lib/support/config.sub $(Machine)-linux)
OS = $(shell echo $(MACHTYPE) | sed 's/^\(.*\)-\([^-]*-[^-]*\)$$/\2/')
VENDOR = pc

CC=gcc
CFLAGS=-Wall -g -fPIC

AR=ar
ARFLAGS=-rcs

TARGET=libvar_params.a

SYSTEM_FLAGS =  -DCONF_HOSTTYPE='"$(Machine)"' -DCONF_OSTYPE='"$(OS)"' -DCONF_MACHTYPE='"$(MACHTYPE)"' 
CFLAGS += $(SYSTEM_FLAGS)


SRCS=$(wildcard *.c)  

OBJS=$(patsubst %.c,%.o,$(SRCS))  

.PHONY: all clean

all: $(TARGET)

$(TARGET): $(OBJS)
	$(AR) $(ARFLAGS) $@ $^

%.o: %.c %.h
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f $(OBJS) $(TARGET)
