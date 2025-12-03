/* common.c - utility functions for all builtins */

/* Copyright (C) 1987-2020 Free Software Foundation, Inc.

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

#if defined (HAVE_UNISTD_H)
#  ifdef _MINIX
#    include <sys/types.h>
#  endif
#  include <unistd.h>
#endif

#include <stdio.h>
#include "chartypes.h"
#include "bashtypes.h"
#include "posixstat.h"
#include <signal.h>

#include <errno.h>

#if defined (PREFER_STDARG)
#  include <stdarg.h>
#else
#  include <varargs.h>
#endif

#include "bashansi.h"
#include "bashintl.h"

#define NEED_FPURGE_DECL

#include "shell.h"
#include "maxpath.h"
#include "flags.h"
#include "parser.h"
#include "jobs.h"
#include "builtins.h"
#include "input.h"
#include "execute_cmd.h"
#include "trap.h"
#include "bashgetopt.h"
#include "common.h"
#include "builtext.h"
#include "tilde.h"

#if defined (HISTORY)
#  include "bashhist.h"
#endif

#if !defined (errno)
extern int errno;   
#endif /* !errno */

extern const char * const bash_getcwd_errstr;

/* Used by some builtins and the mainline code. */
sh_builtin_func_t *last_shell_builtin = (sh_builtin_func_t *)NULL;
sh_builtin_func_t *this_shell_builtin = (sh_builtin_func_t *)NULL;

extern void builtin_error_prolog();

void
#if defined (PREFER_STDARG)
builtin_error (const char *format, ...)
#else
builtin_error (format, va_alist)
     const char *format;
     va_dcl
#endif
{
  va_list args;

  builtin_error_prolog ();

  SH_VA_START (args, format);

  vfprintf (stderr, format, args);
  va_end (args);
  fprintf (stderr, "\n");
}

void
#if defined (PREFER_STDARG)
builtin_warning (const char *format, ...)
#else
builtin_warning (format, va_alist)
     const char *format;
     va_dcl
#endif
{
  va_list args;

  builtin_error_prolog ();
  fprintf (stderr, _("warning: "));

  SH_VA_START (args, format);

  vfprintf (stderr, format, args);
  va_end (args);
  fprintf (stderr, "\n");
}
