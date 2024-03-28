/* history.c, created from history.def. */
#line 22 "./history.def"

#line 58 "./history.def"

#include <config.h>

#if defined (HISTORY)
#include "../bashtypes.h"
#if ! defined(_MINIX) && defined (HAVE_SYS_FILE_H)
#  include <sys/file.h>
#endif
#include "posixstat.h"
#include "filecntl.h"
#include <errno.h>
#include <stdio.h>
#if defined (HAVE_UNISTD_H)
#  include <unistd.h>
#endif

#include "../bashansi.h"
#include "../bashintl.h"

#include "../shell.h"
#include "../flags.h"
#include "../parser.h"
#include "../bashhist.h"
#include <readline/history.h>
#include "bashgetopt.h"
#include "common.h"

#if !defined (errno)
extern int errno;
#endif

static char *histtime PARAMS((HIST_ENTRY *, const char *));
static int display_history PARAMS((WORD_LIST *));
static void push_history PARAMS((WORD_LIST *));
static int expand_and_print_history PARAMS((WORD_LIST *));

#define AFLAG	0x01
#define RFLAG	0x02
#define WFLAG	0x04
#define NFLAG	0x08
#define SFLAG	0x10
#define PFLAG	0x20
#define CFLAG	0x40
#define DFLAG	0x80

extern int r_history_builtin(WORD_LIST *);

int
history_builtin (list)
     WORD_LIST *list;
{
  return  r_history_builtin(list);

 }

/* Accessors for HIST_ENTRY lists that are called HLIST. */
#define histline(i) (hlist[(i)]->line)
#define histdata(i) (hlist[(i)]->data)

static char *
histtime (hlist, histtimefmt)
     HIST_ENTRY *hlist;
     const char *histtimefmt;
{
  static char timestr[128];
  time_t t;
  struct tm *tm;

  t = history_get_time (hlist);
  tm = t ? localtime (&t) : 0;
  if (t && tm)
    strftime (timestr, sizeof (timestr), histtimefmt, tm);
  else if (hlist->timestamp && hlist->timestamp[0])
    snprintf (timestr, sizeof (timestr), _("%s: invalid timestamp"),
	(hlist->timestamp[0] == '#') ? hlist->timestamp + 1: hlist->timestamp);
  else
    strcpy (timestr, "??");
  return timestr;
}

static int
display_history (list)
     WORD_LIST *list;
{
  register int i;
  intmax_t limit;
  HIST_ENTRY **hlist;
  char *histtimefmt, *timestr;

  if (list)
    {
      if (get_numeric_arg (list, 0, &limit) == 0)
	return (EXECUTION_FAILURE);

      if (limit < 0)
	limit = -limit;
    }
  else
    limit = -1;

  hlist = history_list ();

  if (hlist)
    {
      for (i = 0;  hlist[i]; i++)
	;

      if (0 <= limit && limit < i)
	i -= limit;
      else
	i = 0;

      histtimefmt = get_string_value ("HISTTIMEFORMAT");

      while (hlist[i])
	{
	  QUIT;

	  timestr = (histtimefmt && *histtimefmt) ? histtime (hlist[i], histtimefmt) : (char *)NULL;
	  printf ("%5d%c %s%s\n", i + history_base,
		  histdata(i) ? '*' : ' ',
		  ((timestr && *timestr) ? timestr : ""),
		  histline(i));
	  i++;
	}
    }

  return (EXECUTION_SUCCESS);
}

/* Remove the last entry in the history list and add each argument in
   LIST to the history. */
static void
push_history (list)
     WORD_LIST *list;
{
  char *s;

  /* Delete the last history entry if it was a single entry added to the
     history list (generally the `history -s' itself), or if `history -s'
     is being used in a compound command and the compound command was
     added to the history as a single element (command-oriented history).
     If you don't want history -s to remove the compound command from the
     history, change #if 0 to #if 1 below. */
#if 0
  if (remember_on_history && hist_last_line_pushed == 0 &&
	hist_last_line_added && bash_delete_last_history () == 0)
#else
  if (remember_on_history && hist_last_line_pushed == 0 &&
	(hist_last_line_added ||
	  (current_command_line_count > 0 && current_command_first_line_saved && command_oriented_history))
      && bash_delete_last_history () == 0)
#endif
      return;

  s = string_list (list);
  /* Call check_add_history with FORCE set to 1 to skip the check against
     current_command_line_count.  If history -s is used in a compound
     command, the above code will delete the compound command's history
     entry and this call will add the line to the history as a separate
     entry.  Without FORCE=1, if current_command_line_count were > 1, the
     line would be appended to the entry before the just-deleted entry. */
  check_add_history (s, 1);	/* obeys HISTCONTROL, HISTIGNORE */

  hist_last_line_pushed = 1;	/* XXX */
  free (s);
}

#if defined (BANG_HISTORY)
static int
expand_and_print_history (list)
     WORD_LIST *list;
{
  char *s;
  int r, result;

  if (hist_last_line_pushed == 0 && hist_last_line_added && bash_delete_last_history () == 0)
    return EXECUTION_FAILURE;
  result = EXECUTION_SUCCESS;
  while (list)
    {
      r = history_expand (list->word->word, &s);
      if (r < 0)
	{
	  builtin_error (_("%s: history expansion failed"), list->word->word);
	  result = EXECUTION_FAILURE;
	}
      else
	{
	  fputs (s, stdout);
	  putchar ('\n');
	}
      FREE (s);
      list = list->next;
    }
  fflush (stdout);
  return result;
}
#endif /* BANG_HISTORY */
#endif /* HISTORY */
