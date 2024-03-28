//# This file was modified by UnionTech Software Technology Co., Ltd. in 2023/05/30 
/* print_command -- A way to make readable commands from a command tree. */

/* Copyright (C) 1989-2020 Free Software Foundation, Inc.

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

#include <stdio.h>

#if defined (HAVE_UNISTD_H)
#  ifdef _MINIX
#    include <sys/types.h>
#  endif
#  include <unistd.h>
#endif

#if defined (PREFER_STDARG)
#  include <stdarg.h>
#else
#  include <varargs.h>
#endif

#include "bashansi.h"
#include "bashintl.h"

#define NEED_XTRACE_SET_DECL

#include "shell.h"
#include "flags.h"
#include <y.tab.h>	/* use <...> so we pick it up from the build directory */
#include "input.h"

#include "shmbutil.h"

#include "builtins/common.h"

#if !HAVE_DECL_PRINTF
extern int printf PARAMS((const char *, ...));	/* Yuck.  Double yuck. */
#endif


#if defined (PREFER_STDARG)

extern void cprintf PARAMS((const char *, ...))  __attribute__((__format__ (printf, 1, 2)));
static void xprintf PARAMS((const char *, ...))  __attribute__((__format__ (printf, 1, 2)));
#else
#define PFUNC VFunction
static void cprintf ();
static void xprintf ();
#endif

static void the_printed_command_resize PARAMS((int));


extern char *the_printed_command ;
extern int the_printed_command_size ;
extern int command_string_index ;

extern char *get_host_type PARAMS((void));
extern char *get_os_type PARAMS((void));
extern char *get_mach_type PARAMS((void));

/* How to make the string. */
extern void
#if defined (PREFER_STDARG)
cprintf (const char *control, ...)
#else
cprintf (control, va_alist)
     const char *control;
     va_dcl
#endif
{
  register const char *s;
  char char_arg[2], *argp, intbuf[INT_STRLEN_BOUND (unsigned int) + 1];
  int digit_arg, arg_len, c;
  va_list args;

  SH_VA_START (args, control);

  arg_len = strlen (control);
  the_printed_command_resize (arg_len + 1);

  char_arg[1] = '\0';
  s = control;
  while (s && *s)
    {
      c = *s++;
      argp = (char *)NULL;
      if (c != '%' || !*s)
	{
	  char_arg[0] = c;
	  argp = char_arg;
	  arg_len = 1;
	}
      else
	{
	  c = *s++;
	  switch (c)
	    {
	    case '%':
	      char_arg[0] = c;
	      argp = char_arg;
	      arg_len = 1;
	      break;

	    case 's':
	      argp = va_arg (args, char *);
	      arg_len = strlen (argp);
	      break;

	    case 'd':
	      /* Represent an out-of-range file descriptor with an out-of-range
		 integer value.  We can do this because the only use of `%d' in
		 the calls to cprintf is to output a file descriptor number for
		 a redirection. */
	      digit_arg = va_arg (args, int);
	      if (digit_arg < 0)
		{
		  sprintf (intbuf, "%u", (unsigned int)-1);
		  argp = intbuf;
		}
	      else
		argp = inttostr (digit_arg, intbuf, sizeof (intbuf));
	      arg_len = strlen (argp);
	      break;

	    case 'c':
	      char_arg[0] = va_arg (args, int);
	      argp = char_arg;
	      arg_len = 1;
	      break;

	    default:
	      programming_error (_("cprintf: `%c': invalid format character"), c);
	      /*NOTREACHED*/
	    }
	}

      if (argp && arg_len)
	{
	  the_printed_command_resize (arg_len + 1);
	  FASTCOPY (argp, the_printed_command + command_string_index, arg_len);
	  command_string_index += arg_len;
	}
    }

  va_end (args);

  the_printed_command[command_string_index] = '\0';
}

char * 
get_host_type()
{
  return HOSTTYPE;
}

char * 
get_os_type()
{
  return OSTYPE;
}

char * 
get_mach_type()
{
  return MACHTYPE;
}

