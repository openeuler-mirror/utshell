export utshell_dir=/home/tong/src/bash;
export euler_dir=/home/tong/src/openeuler/utshell;
export count=1;
echo $euler_dir
echo $utshell_dir
while read line ; do
let count=count+1;
echo $line|awk -F'$' '{
print "echo cd " ENVIRON["euler_dir"] ";";
print "read ";
}' ; done < ../1 >./x.sh
