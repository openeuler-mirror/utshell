#
# Makefile for the Bash library
#
#
# Copyright (C) 1998-2020 Free Software Foundation, Inc.

#   This program is free software: you can redistribute it and/or modify
#   it under the terms of the GNU General Public License as published by
#   the Free Software Foundation, either version 3 of the License, or
#   (at your option) any later version.

#   This program is distributed in the hope that it will be useful,
#   but WITHOUT ANY WARRANTY; without even the implied warranty of
#   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#   GNU General Public License for more details.

#   You should have received a copy of the GNU General Public License
#   along with this program.  If not, see <http://www.gnu.org/licenses/>.

PACKAGE = utshell
VERSION = 0.1-release

PACKAGE_BUGREPORT = bug-utshell@gnu.org
PACKAGE_NAME = utshell
PACKAGE_STRING = utshell 0.1-release
PACKAGE_VERSION = 0.1-release

srcdir = .

topdir = ../
topdiri = ../include
BUILD_DIR = ../include

LIBBUILD = ${BUILD_DIR}/lib

BASHINCDIR = ${topdir}/include

INTL_LIBSRC = ${topdir}/lib/intl
INTL_BUILDDIR = ${LIBBUILD}/intl
INTL_INC = 
LIBINTL_H = 

datarootdir = ${prefix}/share

INSTALL = /usr/bin/install -c
INSTALL_PROGRAM = ${INSTALL}
INSTALL_DATA = ${INSTALL} -m 644

CC = gcc
RANLIB = ranlib
AR = ar
ARFLAGS = cr
RM = rm -f
CP = cp
MV = mv

SHELL = /bin/sh

CFLAGS = -g -O2 -Wno-parentheses -Wno-format-security -fPIC
LOCAL_CFLAGS =  ${DEBUG}
CPPFLAGS = 
LDFLAGS =  -L./lib/termcap -rdynamic

PROFILE_FLAGS = 

DEFS = -DHAVE_CONFIG_H
LOCAL_DEFS = -DSHELL

#INCLUDES = -I. -I../include -I$(topdir)/include -I$(topdir)/lib -I$(BASHINCDIR) -I$(srcdir) $(INTL_INC)
INCLUDES = -I. -I$(BASHINCDIR) -I$(srcdir) $(INTL_INC)

CCFLAGS = ${ADDON_CFLAGS} ${PROFILE_FLAGS} ${INCLUDES} $(DEFS) $(LOCAL_DEFS) \
	  $(LOCAL_CFLAGS) $(CFLAGS) $(CPPFLAGS) 

GCC_LINT_FLAGS = -Wall -Wshadow -Wpointer-arith -Wcast-qual \
		 -Wcast-align -Wstrict-prototypes -Wconversion \
		 -Wmissing-prototypes -Wtraditional -Wredundant-decls -pedantic

.c.o:
	$(CC) -c $(CCFLAGS) $<

# The name of the library target.
LIBRARY_NAME = libsh.a

# The C code source files for this library.
CSOURCES = clktck.c clock.c getcwd.c getenv.c oslib.c setlinebuf.c \
	   strcasecmp.c strerror.c strtod.c strtol.c strtoul.c \
	   vprint.c itos.c rename.c zread.c zwrite.c shtty.c \
	   inet_aton.c netconn.c netopen.c strpbrk.c timeval.c makepath.c \
	   pathcanon.c pathphys.c tmpfile.c stringlist.c stringvec.c spell.c \
	   shquote.c strtrans.c strcasestr.c snprintf.c mailstat.c \
	   fmtulong.c fmtullong.c fmtumax.c shmatch.c strnlen.c \
	   strtoll.c strtoull.c strtoimax.c strtoumax.c memset.c strstr.c \
	   mktime.c strftime.c mbschr.c zcatfd.c zmapfd.c winsize.c eaccess.c \
	   wcsdup.c fpurge.c zgetline.c mbscmp.c uconvert.c ufuncs.c \
	   casemod.c dprintf.c input_avail.c mbscasecmp.c fnxform.c \
	   strchrnul.c unicode.c wcswidth.c wcsnwidth.c shmbchar.c strdup.c \
	   utf8.c random.c gettimeofday.c

# The header files for this library.
HSOURCES = 

# The object files contained in $(LIBRARY_NAME)
LIBOBJS =  ${LIBOBJDIR}mbschr$U.o
OBJECTS = clktck.o clock.o getenv.o oslib.o setlinebuf.o strnlen.o \
	  itos.o zread.o zwrite.o shtty.o shmatch.o eaccess.o \
	  netconn.o netopen.o timeval.o makepath.o pathcanon.o \
	  pathphys.o tmpfile.o stringlist.o stringvec.o spell.o shquote.o \
	  strtrans.o snprintf.o mailstat.o fmtulong.o \
	  fmtullong.o fmtumax.o zcatfd.o zmapfd.o winsize.o wcsdup.o \
	  fpurge.o zgetline.o mbscmp.o uconvert.o ufuncs.o casemod.o \
	  input_avail.o mbscasecmp.o fnxform.o unicode.o shmbchar.o \
	  utf8.o random.o gettimeofday.o wcsnwidth.o ${LIBOBJS}

SUPPORT = Makefile

all: $(LIBRARY_NAME)

$(LIBRARY_NAME): $(OBJECTS)
	$(RM) $@
	$(AR) $(ARFLAGS) $@ $(OBJECTS)
	-test -n "$(RANLIB)" && $(RANLIB) $@

force:

# The rule for 'includes' is written funny so that the if statement
# always returns TRUE unless there really was an error installing the
# include files.
install:

clean:
	$(RM) $(OBJECTS) $(LIBRARY_NAME)

realclean distclean maintainer-clean: clean
	$(RM) Makefile

mostlyclean: clean

# Dependencies

${BUILD_DIR}/version.h: ${BUILD_DIR}/config.h ${BUILD_DIR}/Makefile Makefile
	-( cd ${BUILD_DIR} && ${MAKE} ${MFLAGS} version.h )

#${BUILD_DIR}/pathnames.h: ${BUILD_DIR}/config.h ${BUILD_DIR}/Makefile Makefile
#	-( cd ${BUILD_DIR} && ${MAKE} ${MFLAGS} pathnames.h )

# rules for losing makes, like SunOS
casemod.o: casemod.c
clktck.o: clktck.c
clock.o: clock.c
eaccess.o: eaccess.c
dprintf.o: dprintf.c
fmtullong.o: fmtullong.c
fmtulong.o: fmtulong.c
fmtumax.o: fmtumax.c
fnxform.o: fnxform.c
fpurge.o: fpurge.c
getcwd.o: getcwd.c
getenv.o: getenv.c
gettimeofday.o: gettimeofday.c
inet_aton.o: inet_aton.c
input_avail.o: input_avail.c
itos.o: itos.c
mailstat.o: mailstat.c
makepath.o: makepath.c
mbscasecmp.o: mbscasecmp.c
mbschr.o: mbschr.c
mbscmp.o: mbscmp.c
memset.o: memset.c
mktime.o: mktime.c
netconn.o: netconn.c
netopen.o: netopen.c
oslib.o: oslib.c
pathcanon.o: pathcanon.c
pathphys.o: pathphys.c
random.o: random.c
rename.o: rename.c
setlinebuf.o: setlinebuf.c
shmatch.o: shmatch.c
shmbchar.o: shmbchar.c
shquote.o: shquote.c
shtty.o: shtty.c
snprintf.o: snprintf.c
spell.o: spell.c
strcasecmp.o: strcasecmp.c
strchrnul.o: strchrnul.c
strerror.o: strerror.c
strftime.o: strftime.c
strcasestr.o: strcasestr.c
stringlist.o: stringlist.c
stringvec.o: stringvec.c
strnlen.o: strnlen.c
strpbrk.o: strpbrk.c
strtod.o: strtod.c
strtoimax.o: strtoimax.c
strtol.o: strtol.c
strtoll.o: strtoll.c
strtoul.o: strtoul.c
strtoull.o: strtoull.c
strtoumax.o: strtoumax.c
strtrans.o: strtrans.c
times.o: times.c
timeval.o: timeval.c
tmpfile.o: tmpfile.c
uconvert.o: uconvert.c
ufuncs.o: ufuncs.c
unicode.o: unicode.c
utf8.o: utf8.c
vprint.o: vprint.c
wcsdup.o: wcsdup.c
wcsnwidth.o: wcsnwidth.c
wcswidth.o: wcswidth.c
winsize.o: winsize.c
zcatfd.o: zcatfd.c
zmapfd.o: zmapfd.c
zgetline.o: zgetline.c
zread.o: zread.c
zwrite.o: zwrite.c

# dependencies for c files that include other c files
fmtullong.o: fmtulong.c
fmtumax.o: fmtulong.c
strtoll.o: strtol.c
strtoul.o: strtol.c
strtoull.o: strtol.c

# all files in the library depend on config.h
casemod.o: ${BUILD_DIR}/config.h
clktck.o: ${BUILD_DIR}/config.h
clock.o: ${BUILD_DIR}/config.h
eaccess.o: ${BUILD_DIR}/config.h
dprintf.o: ${BUILD_DIR}/config.h
fmtullong.o: ${BUILD_DIR}/config.h
fmtulong.o: ${BUILD_DIR}/config.h
fmtumax.o: ${BUILD_DIR}/config.h
fnxform.o: ${BUILD_DIR}/config.h
fpurge.o: ${BUILD_DIR}/config.h
getcwd.o: ${BUILD_DIR}/config.h
getenv.o: ${BUILD_DIR}/config.h
gettimeofday.o: ${BUILD_DIR}/config.h
inet_aton.o: ${BUILD_DIR}/config.h
input_avail.o: ${BUILD_DIR}/config.h
itos.o: ${BUILD_DIR}/config.h
mailstat.o: ${BUILD_DIR}/config.h
makepath.o: ${BUILD_DIR}/config.h
mbscasecmp.o: ${BUILD_DIR}/config.h
mbschr.o: ${BUILD_DIR}/config.h
mbscmp.o: ${BUILD_DIR}/config.h
memset.o: ${BUILD_DIR}/config.h
mktime.o: ${BUILD_DIR}/config.h
netconn.o: ${BUILD_DIR}/config.h
netopen.o: ${BUILD_DIR}/config.h
oslib.o: ${BUILD_DIR}/config.h
pathcanon.o: ${BUILD_DIR}/config.h
pathphys.o: ${BUILD_DIR}/config.h
random.o: ${BUILD_DIR}/config.h
rename.o: ${BUILD_DIR}/config.h
setlinebuf.o: ${BUILD_DIR}/config.h
shmatch.o: ${BUILD_DIR}/config.h
shmbchar.o: ${BUILD_DIR}/config.h
shquote.o: ${BUILD_DIR}/config.h
shtty.o: ${BUILD_DIR}/config.h
snprintf.o: ${BUILD_DIR}/config.h
spell.o: ${BUILD_DIR}/config.h
strcasecmp.o: ${BUILD_DIR}/config.h
strchrnul.o: ${BUILD_DIR}/config.h
strerror.o: ${BUILD_DIR}/config.h
strftime.o: ${BUILD_DIR}/config.h
strcasestr.o: ${BUILD_DIR}/config.h
stringlist.o: ${BUILD_DIR}/config.h
stringvec.o: ${BUILD_DIR}/config.h
strnlen.o: ${BUILD_DIR}/config.h
strpbrk.o: ${BUILD_DIR}/config.h
strtod.o: ${BUILD_DIR}/config.h
strtoimax.o: ${BUILD_DIR}/config.h
strtol.o: ${BUILD_DIR}/config.h
strtoll.o: ${BUILD_DIR}/config.h
strtoul.o: ${BUILD_DIR}/config.h
strtoull.o: ${BUILD_DIR}/config.h
strtoumax.o: ${BUILD_DIR}/config.h
strtrans.o: ${BUILD_DIR}/config.h
times.o: ${BUILD_DIR}/config.h
timeval.o: ${BUILD_DIR}/config.h
tmpfile.o: ${BUILD_DIR}/config.h ${topdiri}/config-top.h 
uconvert.o: ${BUILD_DIR}/config.h
ufuncs.o: ${BUILD_DIR}/config.h
unicode.o: ${BUILD_DIR}/config.h
utf8.o: ${BUILD_DIR}/config.h
vprint.o: ${BUILD_DIR}/config.h
wcsdup.o: ${BUILD_DIR}/config.h
wcsnwidth.o: ${BUILD_DIR}/config.h
wcswidth.o: ${BUILD_DIR}/config.h
winsize.o: ${BUILD_DIR}/config.h
zcatfd.o: ${BUILD_DIR}/config.h
zgetline.o: ${BUILD_DIR}/config.h
zmapfd.o: ${BUILD_DIR}/config.h
zread.o: ${BUILD_DIR}/config.h
zwrite.o: ${BUILD_DIR}/config.h

clktck.o: ${topdiri}/bashtypes.h

getcwd.o: ${topdir}/bashtypes.h ${topdir}/bashansi.h ${BASHINCDIR}/maxpath.h
getcwd.o: ${BASHINCDIR}/posixstat.h ${BASHINCDIR}/posixdir.h
getcwd.o: ${BASHINCDIR}/memalloc.h ${BASHINCDIR}/ansi_stdlib.h

getenv.o: ${topdir}/include/bashansi.h ${BASHINCDIR}/ansi_stdlib.h
getenv.o: ${topdir}/include/shell.h ${topdir}/include/syntax.h ${topdiri}/bashjmp.h ${BASHINCDIR}/posixjmp.h
getenv.o: ${topdir}/include/command.h ${BASHINCDIR}/stdc.h ${topdiri}/error.h
getenv.o: ${topdir}/include/general.h ${topdiri}/bashtypes.h ${topdiri}/variables.h ${topdiri}/conftypes.h
getenv.o: ${topdir}/include/array.h ${topdiri}/hashlib.h ${topdiri}/quit.h
getenv.o: ${topdir}/include/unwind_prot.h ${topdiri}/dispose_cmd.h
getenv.o: ${topdir}/include/make_cmd.h ${topdiri}/subst.h ${topdiri}/sig.h
getenv.o: ${BUILD_DIR}/pathnames.h ${topdiri}/externs.h
#getenv.o: ${BUILD_DIR}/version.h

inet_aton.o: ${topdir}/bashansi.h ${BASHINCDIR}/ansi_stdlib.h
inet_aton.o: ${BASHINCDIR}/stdc.h

itos.o: ${topdiri}/bashansi.h ${BASHINCDIR}/ansi_stdlib.h
itos.o: ${topdiri}/shell.h ${topdir}/include/syntax.h ${topdiri}/bashjmp.h ${BASHINCDIR}/posixjmp.h
itos.o: ${topdiri}/command.h ${BASHINCDIR}/stdc.h ${topdiri}/error.h
itos.o: ${topdiri}/general.h ${topdiri}/bashtypes.h ${topdiri}/variables.h ${topdiri}/conftypes.h
itos.o: ${topdiri}/array.h ${topdiri}/hashlib.h ${topdiri}/quit.h
itos.o: ${topdiri}/unwind_prot.h ${topdiri}/dispose_cmd.h
itos.o: ${topdiri}/make_cmd.h ${topdiri}/subst.h ${topdiri}/sig.h
itos.o: ${BUILD_DIR}/pathnames.h ${topdiri}/externs.h
#itos.o: ${BUILD_DIR}/version.h

makepath.o: ${topdiri}/bashansi.h ${BASHINCDIR}/ansi_stdlib.h
makepath.o: ${topdiri}/shell.h ${topdiri}/syntax.h ${topdiri}/bashjmp.h ${BASHINCDIR}/posixjmp.h
makepath.o: ${topdiri}/command.h ${BASHINCDIR}/stdc.h ${topdiri}/error.h
makepath.o: ${topdiri}/general.h ${topdiri}/bashtypes.h ${topdiri}/variables.h ${topdiri}/conftypes.h
makepath.o: ${topdiri}/array.h ${topdiri}/hashlib.h ${topdiri}/quit.h
makepath.o: ${topdiri}/unwind_prot.h ${topdiri}/dispose_cmd.h
makepath.o: ${topdiri}/make_cmd.h ${topdiri}/subst.h ${topdiri}/sig.h
makepath.o: ${BUILD_DIR}/pathnames.h ${topdiri}/externs.h
#makepath.o: ${BUILD_DIR}/version.h

netconn.o: ${BASHINCDIR}/posixstat.h ${BASHINCDIR}/filecntl.h
netconn.o: ${topdiri}/bashtypes.h

netopen.o: ${topdiri}/bashansi.h ${BASHINCDIR}/ansi_stdlib.h ${topdiri}/xmalloc.h
netopen.o: ${topdiri}/shell.h ${topdiri}/syntax.h ${topdiri}/bashjmp.h ${BASHINCDIR}/posixjmp.h
netopen.o: ${topdiri}/command.h ${BASHINCDIR}/stdc.h ${topdiri}/error.h
netopen.o: ${topdiri}/general.h ${topdiri}/bashtypes.h ${topdiri}/variables.h ${topdiri}/conftypes.h
netopen.o: ${topdiri}/array.h ${topdiri}/hashlib.h ${topdiri}/quit.h
netopen.o: ${topdiri}/unwind_prot.h ${topdiri}/dispose_cmd.h
netopen.o: ${topdiri}/make_cmd.h ${topdiri}/subst.h ${topdiri}/sig.h
netopen.o: ${BUILD_DIR}/pathnames.h ${topdiri}/externs.h
netopen.o: ${topdiri}/bashintl.h ${LIBINTL_H} $(BASHINCDIR)/gettext.h
#netopen.o: ${BUILD_DIR}/version.h

oslib.o: ${topdiri}/bashtypes.h ${topdiri}/bashansi.h ${BASHINCDIR}/maxpath.h
oslib.o: ${topdiri}/shell.h ${topdiri}/syntax.h ${topdiri}/bashjmp.h ${BASHINCDIR}/posixjmp.h
oslib.o: ${topdiri}/command.h ${BASHINCDIR}/stdc.h ${topdiri}/error.h
oslib.o: ${topdiri}/general.h ${topdiri}/bashtypes.h ${topdiri}/variables.h ${topdiri}/conftypes.h
oslib.o: ${topdiri}/array.h ${topdiri}/hashlib.h ${topdiri}/quit.h
oslib.o: ${topdiri}/unwind_prot.h ${topdiri}/dispose_cmd.h
oslib.o: ${topdiri}/make_cmd.h ${topdiri}/subst.h ${topdiri}/sig.h
oslib.o: ${BUILD_DIR}/pathnames.h ${topdiri}/externs.h
oslib.o: ${BASHINCDIR}/posixstat.h ${BASHINCDIR}/filecntl.h
oslib.o: ${BASHINCDIR}/ansi_stdlib.h ${BASHINCDIR}/chartypes.h
#oslib.o: ${BUILD_DIR}/version.h

pathcanon.o: ${topdiri}/bashtypes.h ${topdiri}/bashansi.h ${BASHINCDIR}/maxpath.h
pathcanon.o: ${topdiri}/shell.h ${topdiri}/syntax.h ${topdiri}/bashjmp.h ${BASHINCDIR}/posixjmp.h
pathcanon.o: ${topdiri}/command.h ${BASHINCDIR}/stdc.h ${topdiri}/error.h
pathcanon.o: ${topdiri}/general.h ${topdiri}/bashtypes.h ${topdiri}/variables.h ${topdiri}/conftypes.h
pathcanon.o: ${topdiri}/array.h ${topdiri}/hashlib.h ${topdiri}/quit.h
pathcanon.o: ${topdiri}/unwind_prot.h ${topdiri}/dispose_cmd.h
pathcanon.o: ${topdiri}/make_cmd.h ${topdiri}/subst.h ${topdiri}/sig.h
pathcanon.o: ${BUILD_DIR}/pathnames.h ${topdiri}/externs.h
pathcanon.o: ${BASHINCDIR}/posixstat.h ${BASHINCDIR}/filecntl.h
pathcanon.o: ${BASHINCDIR}/ansi_stdlib.h ${BASHINCDIR}/chartypes.h
#pathcanon.o: ${BUILD_DIR}/version.h

pathphys.o: ${topdiri}/bashtypes.h ${topdiri}/bashansi.h ${BASHINCDIR}/maxpath.h
pathphys.o: ${topdiri}/shell.h ${topdiri}/syntax.h ${topdiri}/bashjmp.h ${BASHINCDIR}/posixjmp.h
pathphys.o: ${topdiri}/command.h ${BASHINCDIR}/stdc.h ${topdiri}/error.h
pathphys.o: ${topdiri}/general.h ${topdiri}/bashtypes.h ${topdiri}/variables.h ${topdiri}/conftypes.h
pathphys.o: ${topdiri}/array.h ${topdiri}/hashlib.h ${topdiri}/quit.h
pathphys.o: ${topdiri}/unwind_prot.h ${topdiri}/dispose_cmd.h
pathphys.o: ${topdiri}/make_cmd.h ${topdiri}/subst.h ${topdiri}/sig.h
pathphys.o: ${BUILD_DIR}/pathnames.h ${topdiri}/externs.h
pathphys.o: ${BASHINCDIR}/posixstat.h ${BASHINCDIR}/filecntl.h
pathphys.o: ${BASHINCDIR}/ansi_stdlib.h ${BASHINCDIR}/chartypes.h
#pathphys.o: ${BUILD_DIR}/version.h

random.o: ${topdiri}/bashtypes.h ${BASHINCDIR}/stdc.h
random.o: ${topdiri}/bashansi.h ${BASHINCDIR}/ansi_stdlib.h
random.o: ${BASHINCDIR}/filecntl.h

rename.o: ${topdir}/bashtypes.h ${BASHINCDIR}/stdc.h
rename.o: ${BASHINCDIR}/posixstat.h

setlinebuf.o: ${topdiri}/xmalloc.h ${topdiri}/bashansi.h
setlinebuf.o: ${BASHINCDIR}/ansi_stdlib.h ${BASHINCDIR}/stdc.h

eaccess.o: ${topdiri}/bashtypes.h
eaccess.o: ${BASHINCDIR}/posixstat.h
eaccess.o: ${topdiri}/bashansi.h ${BASHINCDIR}/ansi_stdlib.h
eaccess.o: ${BASHINCDIR}/filecntl.h
eaccess.o: ${BASHINCDIR}/stdc.h
eaccess.o: ${topdiri}/shell.h ${topdiri}/syntax.h ${topdiri}/bashjmp.h ${BASHINCDIR}/posixjmp.h
eaccess.o: ${topdiri}/command.h ${BASHINCDIR}/stdc.h ${topdiri}/error.h
eaccess.o: ${topdiri}/general.h ${topdiri}/bashtypes.h ${topdiri}/variables.h ${topdiri}/conftypes.h
eaccess.o: ${topdiri}/array.h ${topdiri}/hashlib.h ${topdiri}/quit.h
eaccess.o: ${topdiri}/unwind_prot.h ${topdiri}/dispose_cmd.h
eaccess.o: ${topdiri}/make_cmd.h ${topdiri}/subst.h ${topdiri}/sig.h
eaccess.o: ${BUILD_DIR}/pathnames.h ${topdiri}/externs.h
#eaccess.o: ${BUILD_DIR}/version.h

shmatch.o: ${BASHINCDIR}/stdc.h ${topdiri}/bashansi.h
shmatch.o: ${BASHINCDIR}/ansi_stdlib.h ${topdiri}/xmalloc.h
shmatch.o: ${topdiri}/shell.h ${topdiri}/syntax.h ${topdiri}/bashjmp.h ${BASHINCDIR}/posixjmp.h
shmatch.o: ${topdiri}/command.h ${BASHINCDIR}/stdc.h ${topdiri}/error.h
shmatch.o: ${topdiri}/general.h ${topdiri}/bashtypes.h ${topdiri}/variables.h ${topdiri}/conftypes.h
shmatch.o: ${topdiri}/array.h ${topdiri}/hashlib.h ${topdiri}/quit.h
shmatch.o: ${topdiri}/unwind_prot.h ${topdiri}/dispose_cmd.h
shmatch.o: ${topdiri}/make_cmd.h ${topdiri}/subst.h ${topdiri}/sig.h
shmatch.o: ${BUILD_DIR}/pathnames.h ${topdiri}/externs.h

shquote.o: ${BASHINCDIR}/stdc.h ${topdiri}/bashansi.h
shquote.o: ${BASHINCDIR}/ansi_stdlib.h ${topdiri}/xmalloc.h
shquote.o: ${BASHINCDIR}/shmbutil.h ${BASHINCDIR}/shmbchar.h

shtty.o: ${BASHINCDIR}/shtty.h
shtty.o: ${BASHINCDIR}/stdc.h

snprintf.o: ${BASHINCDIR}/stdc.h ${topdiri}/bashansi.h ${topdiri}/xmalloc.h
snprintf.o: ${BASHINCDIR}/ansi_stdlib.h ${BASHINCDIR}/chartypes.h
snprintf.o: ${BASHINCDIR}/shmbutil.h ${BASHINCDIR}/shmbchar.h
snprintf.o: ${BASHINCDIR}/typemax.h

spell.o: ${topdiri}/bashtypes.h
spell.o: ${BASHINCDIR}/posixstat.h ${BASHINCDIR}/posixdir.h
spell.o: ${BASHINCDIR}/ansi_stdlib.h

strcasecmp.o: ${BASHINCDIR}/stdc.h ${topdir}/bashansi.h
strcasecmp.o: ${BASHINCDIR}/ansi_stdlib.h ${BASHINCDIR}/chartypes.h

strerror.o: ${topdir}/bashtypes.h
strerror.o: ${topdir}/shell.h ${topdir}/syntax.h ${topdir}/bashjmp.h ${BASHINCDIR}/posixjmp.h
strerror.o: ${topdir}/command.h ${BASHINCDIR}/stdc.h ${topdir}/error.h
strerror.o: ${topdir}/general.h ${topdir}/bashtypes.h ${topdir}/variables.h ${topdir}/conftypes.h
strerror.o: ${topdir}/array.h ${topdir}/hashlib.h ${topdir}/quit.h
strerror.o: ${topdir}/unwind_prot.h ${topdir}/dispose_cmd.h
strerror.o: ${topdir}/make_cmd.h ${topdir}/subst.h ${topdir}/sig.h
strerror.o: ${BUILD_DIR}/pathnames.h ${topdir}/externs.h
#strerror.o: ${BUILD_DIR}/version.h

strcasestr.o: ${BASHINCDIR}/stdc.h ${topdir}/bashansi.h
strcasestr.o: ${BASHINCDIR}/ansi_stdlib.h ${BASHINCDIR}/chartypes.h

stringlist.o: ${topdiri}/bashansi.h
stringlist.o: ${topdiri}/shell.h ${topdiri}/syntax.h ${topdiri}/bashjmp.h ${BASHINCDIR}/posixjmp.h
stringlist.o: ${topdiri}/command.h ${BASHINCDIR}/stdc.h ${topdiri}/error.h
stringlist.o: ${topdiri}/general.h ${topdiri}/bashtypes.h ${topdiri}/variables.h ${topdiri}/conftypes.h
stringlist.o: ${topdiri}/array.h ${topdiri}/hashlib.h ${topdiri}/quit.h
stringlist.o: ${topdiri}/unwind_prot.h ${topdiri}/dispose_cmd.h
stringlist.o: ${topdiri}/make_cmd.h ${topdiri}/subst.h ${topdiri}/sig.h
stringlist.o: ${BUILD_DIR}/pathnames.h ${topdiri}/externs.h
#stringlist.o: ${BUILD_DIR}/version.h

stringvec.o: ${topdiri}/bashansi.h ${BASHINCDIR}/chartypes.h
stringvec.o: ${topdiri}/shell.h ${topdiri}/syntax.h ${topdiri}/bashjmp.h ${BASHINCDIR}/posixjmp.h
stringvec.o: ${topdiri}/command.h ${BASHINCDIR}/stdc.h ${topdiri}/error.h
stringvec.o: ${topdiri}/general.h ${topdiri}/bashtypes.h ${topdiri}/variables.h ${topdiri}/conftypes.h
stringvec.o: ${topdiri}/array.h ${topdiri}/hashlib.h ${topdiri}/quit.h
stringvec.o: ${topdiri}/unwind_prot.h ${topdiri}/dispose_cmd.h
stringvec.o: ${topdiri}/make_cmd.h ${topdiri}/subst.h ${topdiri}/sig.h
stringvec.o: ${BUILD_DIR}/pathnames.h ${topdiri}/externs.h
#stringvec.o: ${BUILD_DIR}/version.h

strnlen.o: ${BASHINCDIR}/stdc.h

strpbrk.o: ${BASHINCDIR}/stdc.h

strtod.o: ${topdir}/bashansi.h
strtod.o: ${BASHINCDIR}/ansi_stdlib.h ${BASHINCDIR}/chartypes.h

strtoimax.o: ${BASHINCDIR}/stdc.h

strtol.o: ${topdir}/bashansi.h
strtol.o: ${BASHINCDIR}/ansi_stdlib.h ${BASHINCDIR}/chartypes.h
strtol.o: ${BASHINCDIR}/typemax.h

strtoll.o: ${topdir}/bashansi.h
strtoll.o: ${BASHINCDIR}/ansi_stdlib.h ${BASHINCDIR}/chartypes.h
strtoll.o: ${BASHINCDIR}/typemax.h

strtoul.o: ${topdir}/bashansi.h
strtoul.o: ${BASHINCDIR}/ansi_stdlib.h ${BASHINCDIR}/chartypes.h
strtoul.o: ${BASHINCDIR}/typemax.h

strtoull.o: ${topdir}/bashansi.h
strtoull.o: ${BASHINCDIR}/ansi_stdlib.h ${BASHINCDIR}/chartypes.h
strtoull.o: ${BASHINCDIR}/typemax.h

strtoumax.o: ${BASHINCDIR}/stdc.h

strtrans.o: ${topdiri}/bashansi.h
strtrans.o: ${BASHINCDIR}/ansi_stdlib.h ${BASHINCDIR}/chartypes.h
strtrans.o: ${topdiri}/shell.h ${topdiri}/syntax.h ${topdiri}/bashjmp.h ${BASHINCDIR}/posixjmp.h
strtrans.o: ${topdiri}/command.h ${BASHINCDIR}/stdc.h ${topdiri}/error.h
strtrans.o: ${topdiri}/general.h ${topdiri}/bashtypes.h ${topdiri}/variables.h ${topdiri}/conftypes.h
strtrans.o: ${topdiri}/array.h ${topdiri}/hashlib.h ${topdiri}/quit.h
strtrans.o: ${topdiri}/unwind_prot.h ${topdiri}/dispose_cmd.h
strtrans.o: ${topdiri}/make_cmd.h ${topdiri}/subst.h ${topdiri}/sig.h
strtrans.o: ${BUILD_DIR}/pathnames.h ${topdiri}/externs.h
strtrans.o: ${BASHINCDIR}/shmbutil.h ${BASHINCDIR}/shmbchar.h
#strtrans.o: ${BUILD_DIR}/version.h

times.o: ${BASHINCDIR}/systimes.h
times.o: ${BASHINCDIR}/posixtime.h

timeval.o: ${BASHINCDIR}/posixtime.h
gettimeofday.o: ${BASHINCDIR}/posixtime.h

tmpfile.o: ${topdiri}/bashtypes.h
tmpfile.o: ${BASHINCDIR}/chartypes.h
tmpfile.o: ${BASHINCDIR}/posixstat.h
tmpfile.o: ${BASHINCDIR}/filecntl.h
tmpfile.o: ${topdiri}/shell.h ${topdiri}/syntax.h ${topdiri}/bashjmp.h ${BASHINCDIR}/posixjmp.h
tmpfile.o: ${topdiri}/command.h ${BASHINCDIR}/stdc.h ${topdiri}/error.h
tmpfile.o: ${topdiri}/general.h ${topdiri}/bashtypes.h ${topdiri}/variables.h ${topdiri}/conftypes.h
tmpfile.o: ${topdiri}/array.h ${topdiri}/hashlib.h ${topdiri}/quit.h
tmpfile.o: ${topdiri}/unwind_prot.h ${topdiri}/dispose_cmd.h
tmpfile.o: ${topdiri}/make_cmd.h ${topdiri}/subst.h ${topdiri}/sig.h
tmpfile.o: ${BUILD_DIR}/pathnames.h ${topdiri}/externs.h

uconvert.o: ${topdiri}/bashtypes.h
uconvert.o: ${BASHINCDIR}/chartypes.h
uconvert.o: ${topdiri}/shell.h ${topdiri}/syntax.h ${topdiri}/bashjmp.h ${BASHINCDIR}/posixjmp.h
uconvert.o: ${topdiri}/command.h ${BASHINCDIR}/stdc.h ${topdiri}/error.h
uconvert.o: ${topdiri}/general.h ${topdiri}/bashtypes.h ${topdiri}/variables.h ${topdiri}/conftypes.h
uconvert.o: ${topdiri}/array.h ${topdiri}/hashlib.h ${topdiri}/quit.h
uconvert.o: ${topdiri}/unwind_prot.h ${topdiri}/dispose_cmd.h
uconvert.o: ${topdiri}/make_cmd.h ${topdiri}/subst.h ${topdiri}/sig.h
uconvert.o: ${BUILD_DIR}/pathnames.h ${topdiri}/externs.h

ufuncs.o: ${topdiri}/bashtypes.h

clock.o: ${BASHINCDIR}/posixtime.h

mailstat.o: ${topdiri}/bashansi.h
mailstat.o: ${topdiri}/bashtypes.h
mailstat.o: ${BASHINCDIR}/ansi_stdlib.h
mailstat.o: ${BASHINCDIR}/posixstat.h
mailstat.o: ${BASHINCDIR}/posixdir.h
mailstat.o: ${BASHINCDIR}/maxpath.h

fmtulong.o: ${topdiri}/bashansi.h
fmtulong.o: ${BASHINCDIR}/ansi_stdlib.h
fmtulong.o: ${BASHINCDIR}/chartypes.h
fmtulong.o: ${BASHINCDIR}/stdc.h
fmtulong.o: ${BASHINCDIR}/typemax.h
fmtulong.o: ${topdiri}/bashintl.h ${LIBINTL_H} ${BASHINCDIR}/gettext.h

fmtullong.o: ${topdiri}/bashansi.h
fmtullong.o: ${BASHINCDIR}/ansi_stdlib.h
fmtullong.o: ${BASHINCDIR}/chartypes.h
fmtullong.o: ${BASHINCDIR}/stdc.h
fmtullong.o: ${BASHINCDIR}/typemax.h
fmtullong.o: ${topdiri}/bashintl.h ${LIBINTL_H} ${BASHINCDIR}/gettext.h

fmtumax.o: ${topdiri}/bashansi.h
fmtumax.o: ${BASHINCDIR}/ansi_stdlib.h
fmtumax.o: ${BASHINCDIR}/chartypes.h
fmtumax.o: ${BASHINCDIR}/stdc.h
fmtumax.o: ${BASHINCDIR}/typemax.h
fmtumax.o: ${topdiri}/bashintl.h ${LIBINTL_H} ${BASHINCDIR}/gettext.h

wcsdup.o: ${topdiri}/bashansi.h ${BASHINCDIR}/ansi_stdlib.h
wcsdup.o: ${BASHINCDIR}/stdc.h
wcsdup.o: ${topdiri}/xmalloc.h

wcsnwidth.o: ${topdiri}/bashansi.h ${BASHINCDIR}/ansi_stdlib.h
wcsnwidth.o: ${BASHINCDIR}/stdc.h

wcswidth.o: ${topdiri}/bashansi.h ${BASHINCDIR}/ansi_stdlib.h
wcswidth.o: ${BASHINCDIR}/stdc.h

mbschr.o: ${topdiri}/bashansi.h
mbschr.o: ${BASHINCDIR}/ansi_stdlib.h
mbschr.o: ${BASHINCDIR}/shmbutil.h ${BASHINCDIR}/shmbchar.h

zgetline.o: ${topdiri}/bashansi.h ${BASHINCDIR}/ansi_stdlib.h
zgetline.o: ${BASHINCDIR}/stdc.h
zgetline.o: ${topdiri}/xmalloc.h
zgetline.o: ${topdiri}/bashtypes.h

mbscasecmp.o: ${topdiri}/bashansi.h ${BASHINCDIR}/ansi_stdlib.h
mbscasecmp.o: ${BASHINCDIR}/stdc.h
mbscasecmp.o: ${topdiri}/xmalloc.h

mbscmp.o: ${topdiri}/bashansi.h ${BASHINCDIR}/ansi_stdlib.h
mbscmp.o: ${BASHINCDIR}/stdc.h
mbscmp.o: ${topdiri}/xmalloc.h

casemod.o: ${topdiri}/bashansi.h ${BASHINCDIR}/ansi_stdlib.h
casemod.o: ${BASHINCDIR}/stdc.h
casemod.o: ${topdiri}/xmalloc.h
casemod.o: ${topdiri}/bashtypes.h
casemod.o: ${BASHINCDIR}/shmbutil.h ${BASHINCDIR}/shmbchar.h
casemod.o: ${topdiri}/bashintl.h ${LIBINTL_H} ${BASHINCDIR}/gettext.h

dprintf.o: ${BASHINCDIR}/stdc.h

input_avail.o: ${topdiri}/bashansi.h ${BASHINCDIR}/ansi_stdlib.h
input_avail.o: ${BASHINCDIR}/stdc.h
input_avail.o: ${topdiri}/xmalloc.h ${BASHINCDIR}/posixselect.h

mktime.o: ${topdiri}/bashansi.h ${BASHINCDIR}/ansi_stdlib.h
mktime.o: ${BASHINCDIR}/stdc.h

fnxform.o: ${topdiri}/bashansi.h ${BASHINCDIR}/ansi_stdlib.h
fnxform.o: ${BASHINCDIR}/stdc.h
fnxform.o: ${topdiri}/bashtypes.h
fnxform.o: ${topdiri}/bashintl.h ${LIBINTL_H} ${BASHINCDIR}/gettext.h

shmbchar.o: ${BASHINCDIR}/shmbchar.h
shmbchar.o: ${BASHINCDIR}/shmbutil.h

unicode.o: ${topdiri}/bashansi.h ${BASHINCDIR}/ansi_stdlib.h
unicode.o: ${BASHINCDIR}/stdc.h
unicode.o: ${topdiri}/xmalloc.h

utf8.o: ${topdiri}/bashansi.h
utf8.o: ${BASHINCDIR}/ansi_stdlib.h
utf8.o: ${BASHINCDIR}/shmbutil.h ${BASHINCDIR}/shmbchar.h

winsize.o: ${BASHINCDIR}/stdc.h
winsize.o: ${topdiri}/xmalloc.h
winsize.o: ${topdiri}/bashtypes.h

zmapfd.o: ${topdiri}/bashansi.h ${BASHINCDIR}/ansi_stdlib.h
zmapfd.o: ${BASHINCDIR}/stdc.h
zmapfd.o: ${topdiri}/command.h
zmapfd.o: ${topdiri}/general.h
zmapfd.o: ${topdiri}/bashtypes.h ${BASHINCDIR}/chartypes.h ${topdiri}/xmalloc.h
