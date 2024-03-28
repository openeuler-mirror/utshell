#
# Regular cron jobs for the utshell package
#
0 4	* * *	root	[ -x /usr/bin/utshell_maintenance ] && /usr/bin/utshell_maintenance
