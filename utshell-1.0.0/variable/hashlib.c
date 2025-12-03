//# This file was modified by UnionTech Software Technology Co., Ltd. in 2023/05/30 
/* hashlib.c -- functions to manage and access hash tables for bash. */

/* Copyright (C) 1987,1989,1991,1995,1998,2001,2003,2005,2006,2008,2009 Free Software Foundation, Inc.

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

#include "bashansi.h"

#if defined (HAVE_UNISTD_H)
#  ifdef _MINIX
#    include <sys/types.h>
#  endif
#  include <unistd.h>
#endif

#include <stdio.h>

#include "shell.h"
#include "hashlib.h"

/* tunable constants for rehashing */
#define HASH_REHASH_MULTIPLIER	4
#define HASH_REHASH_FACTOR	2

#define HASH_SHOULDGROW(table) \
  ((table)->nentries >= (table)->nbuckets * HASH_REHASH_FACTOR)

/* an initial approximation */
#define HASH_SHOULDSHRINK(table) \
  (((table)->nbuckets > DEFAULT_HASH_BUCKETS) && \
   ((table)->nentries < (table)->nbuckets / HASH_REHASH_MULTIPLIER))

/* Rely on properties of unsigned division (unsigned/int -> unsigned) and
   don't discard the upper 32 bits of the value, if present. */
#define HASH_BUCKET(s, t, h) (((h) = hash_string (s)) & ((t)->nbuckets - 1))

#define FNV_OFFSET 2166136261
#define FNV_PRIME 16777619



#if defined (DEBUG) || defined (TEST_HASHING)
void
hash_pstats (table, name)
     HASH_TABLE *table;
     char *name;
{
  register int slot, bcount;
  register BUCKET_CONTENTS *bc;

  if (name == 0)
    name = "unknown hash table";

  fprintf (stderr, "%s: %d buckets; %d items\n", name, table->nbuckets, table->nentries);

  /* Print out a count of how many strings hashed to each bucket, so we can
     see how even the distribution is. */
  for (slot = 0; slot < table->nbuckets; slot++)
    {
      bc = hash_items (slot, table);

      fprintf (stderr, "\tslot %3d: ", slot);
      for (bcount = 0; bc; bc = bc->next)
	bcount++;

      fprintf (stderr, "%d\n", bcount);
    }
}
#endif

#ifdef TEST_HASHING

/* link with xmalloc.o and lib/malloc/libmalloc.a */
#undef NULL
#include <stdio.h>

#ifndef NULL
#define NULL 0
#endif

HASH_TABLE *table, *ntable;

int interrupt_immediately = 0;
int running_trap = 0;

int
signal_is_trapped (s)
     int s;
{
  return (0);
}

void
programming_error (const char *format, ...)
{
  abort();
}

void
fatal_error (const char *format, ...)
{
  abort();
}

void
internal_warning (const char *format, ...)
{
}

int
main ()
{
  char string[256];
  int count = 0;
  BUCKET_CONTENTS *tt;

#if defined (TEST_NBUCKETS)
  table = hash_create (TEST_NBUCKETS);
#else
  table = hash_create (0);
#endif

  for (;;)
    {
      char *temp_string;
      if (fgets (string, sizeof (string), stdin) == 0)
	break;
      if (!*string)
	break;
      temp_string = savestring (string);
      tt = hash_insert (temp_string, table, 0);
      if (tt->times_found)
	{
	  fprintf (stderr, "You have already added item `%s'\n", string);
	  free (temp_string);
	  temp_string = NULL
	}
      else
	{
	  count++;
	}
    }

  hash_pstats (table, "hash test");

  ntable = hash_copy (table, (sh_string_func_t *)NULL);
  hash_flush (table, (sh_free_func_t *)NULL);
  hash_pstats (ntable, "hash copy test");

  exit (0);
}

#endif /* TEST_HASHING */
