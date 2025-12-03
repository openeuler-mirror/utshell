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

/* printf -v var support */
static char *vbuf;
static size_t vbsize;
static int vblen;

static int
#if defined (PREFER_STDARG)
vbprintf (const char *format, ...)
#else
vbprintf (format, va_alist)
  const char *format;
  va_dcl
#endif
{
  va_list args;
  size_t nlen;
  int blen;

  SH_VA_START (args, format);
  blen = vsnprintf (vbuf + vblen, vbsize - vblen, format, args);
  va_end (args);

  nlen = vblen + blen + 1;
  if (nlen >= vbsize)
    {
      vbsize = ((nlen + 63) >> 6) << 6;
      vbuf = (char *)xrealloc (vbuf, vbsize);
      SH_VA_START (args, format);
      blen = vsnprintf (vbuf + vblen, vbsize - vblen, format, args);
      va_end (args);
    }

  vblen += blen;
  vbuf[vblen] = '\0';

#ifdef DEBUG
  if  (strlen (vbuf) != vblen)
    internal_error  ("printf:vbprintf: vblen (%d) != strlen (vbuf) (%d)", vblen, (int)strlen (vbuf));
#endif
  
  return (blen);
}


