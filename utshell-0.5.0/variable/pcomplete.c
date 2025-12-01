//# This file was modified by UnionTech Software Technology Co., Ltd. in 2023/05/30 
/* pcomplete.c - functions to generate lists of matches for programmable completion. */

/* Copyright (C) 1999-2020 Free Software Foundation, Inc.

   This file is part of GNU Bash, the Bourne Again SHell.

   Bash is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   Bash is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with Bash.  If not, see <http://www.gnu.org/licenses/>.
*/

#include "config.h"

#if defined (PROGRAMMABLE_COMPLETION)

#include "bashtypes.h"
#include "posixstat.h"

#if defined (HAVE_UNISTD_H)
#  include <unistd.h>
#endif

#include <signal.h>

#if defined (PREFER_STDARG)
#  include <stdarg.h>
#else
#  include <varargs.h>
#endif

#include "posixtime.h"

#include <stdio.h>
#include "bashansi.h"
#include "bashintl.h"

#include "shell.h"
#include "pcomplete.h"
#include "alias.h"
#include "bashline.h"
#include "execute_cmd.h"
#include "pathexp.h"

#if defined (JOB_CONTROL)
#  include "jobs.h"
#endif

#if !defined (NSIG)
#  include "trap.h"
#endif

#include "shmbutil.h"

#include "builtins.h"
#include "common.h"
#include "builtext.h"

#include <glob.h>
#include "strmatch.h"

#include "rlconf.h"
#include "readline.h"
#include "history.h"

#ifdef STRDUP
#  undef STRDUP
#endif
#define STRDUP(x)	((x) ? savestring (x) : (char *)NULL)

typedef SHELL_VAR **SVFUNC ();

#ifndef HAVE_STRPBRK
extern char *strpbrk PARAMS((char *, char *));
#endif

extern STRING_INT_ALIST word_token_alist[];
extern char *signal_names[];

#if defined (DEBUG)
#if defined (PREFER_STDARG)
static void debug_printf (const char *, ...)  __attribute__((__format__ (printf, 1, 2)));
#endif
#endif /* DEBUG */



#ifdef DEBUG
/* Debugging code */
static void
#if defined (PREFER_STDARG)
debug_printf (const char *format, ...)
#else
debug_printf (format, va_alist)
     const char *format;
     va_dcl
#endif
{
  va_list args;

  if (progcomp_debug == 0)
    return;

  SH_VA_START (args, format);

  fprintf (stdout, "DEBUG: ");
  vfprintf (stdout, format, args);
  fprintf (stdout, "\n");

  rl_on_new_line ();

  va_end (args);
}
#endif
#endif