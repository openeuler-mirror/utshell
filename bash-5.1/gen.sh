export utshell_dir=/home/huan/gerrit/bash;
export euler_dir=/home/huan/gerrit/utshell;
export count=0`tail -1 ${euler_dir}/record.txt`;
echo $euler_dir
echo $utshell_dir
while read line ; do
let count=count+1;
echo $line|awk -F'$' '{
print "echo cd " ENVIRON["euler_dir"] ";";
print "cd " ENVIRON["euler_dir"] ";";
print "echo git checkout -b bv" ENVIRON["count"] ";";
print "git checkout -b bv" ENVIRON["count"] ";";
print "cd " ENVIRON["utshell_dir"] ";";
print "echo cd " ENVIRON["utshell_dir"] ";";
print "git reset --hard ",$1," ; ";
print "echo git reset --hard ",$1," ; ";
print "cp -r " ENVIRON["utshell_dir"]  "//* " ENVIRON["euler_dir"]  ";"
print "echo cp -r " ENVIRON["utshell_dir"]  "//*  " ENVIRON["euler_dir"] ";"
print "cd " ENVIRON["euler_dir"];
print "echo cd " ENVIRON["euler_dir"] ";"
print "echo " ENVIRON["count"] " >> record.txt";
print "echo echo " $1 " to record.txt ; ";
print "git add .;"
print "echo  git add .;"
print "git commit -m \""  $2 "\"";
print "echo git commit -m "  $2," ;";
print "git push  origin bv" ENVIRON["count"] ;
print "echo git push  origin bv" ENVIRON["count"] ";";
print "cd  " ENVIRON["utshell_dir"];
print "echo cd  " ENVIRON["utshell_dir"] ";"
print "read ";
}' ; done < ../1 >./x.sh
