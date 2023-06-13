#!/bin/bash
find .  -name "*.o" > o.list
for line in `cat o.list`
do
   
   soname=${line%%.o*}
   soname=${soname##*.}
   soname=${soname////_}   
   gcc -shared -fPIC -o /opt/rsbash/builtins/lib${soname}.so $line    
done
