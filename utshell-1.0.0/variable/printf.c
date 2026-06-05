#include "config.h"

#include "bashtypes.h"

#include <errno.h>
#if defined (HAVE_LIMITS_H)
#  include <limits.h>
#else
   /* Assume 32-bit ints. */
#  define INT_MAX		2147483647
#  define INT_MIN		(-2147483647-1)
#endif

#if defined (PREFER_STDARG)
#  include <stdarg.h>
#else
#  include <varargs.h>
#endif

#include <stdio.h>
#include "chartypes.h"

#ifdef HAVE_INTTYPES_H
#  include <inttypes.h>
#endif

#include "posixtime.h"
#include "bashansi.h"
#include "bashintl.h"

#define NEED_STRFTIME_DECL

#include "shell.h"
#include "shmbutil.h"
#include "stdc.h"
#include "bashgetopt.h"
#include "common.h"


#if !HAVE_ASPRINTF
extern int asprintf PARAMS((char **, const char *, ...)) __attribute__((__format__ (printf, 2, 3)));
#endif

#if !HAVE_VSNPRINTF
extern int vsnprintf PARAMS((char *, size_t, const char *, va_list)) __attribute__((__format__ (printf, 3, 0)));
#endif


#if defined (HAVE_LONG_DOUBLE) && HAVE_DECL_STRTOLD && !defined(STRTOLD_BROKEN)
typedef long double floatmax_t;
#  define FLOATMAX_CONV	"L"
#  define strtofltmax	strtold
#else
typedef double floatmax_t;
#  define FLOATMAX_CONV	""
#  define strtofltmax	strtod
#endif




