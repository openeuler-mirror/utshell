#!/bin/bash
while read line
do 
	sed -i '1i use crate::src_common::*; ' ${line}
#	sed -i 's/use crate::src_common::\*;//g' $line
done < 1.txt 
