cd /home/huan/gerrit/bash/
git log --pretty=format:"%H $ %s" >2
cat 2|tac >1

while read line ; do echo $line|awk -F'$' '{ 
print " cd /home/huan/gerrit/utshell/ ; ";
print " git checkout master;  git pull ;";
print " git checkout -b " $1 " ; ";
print " cd /home/tong/src/bash/ ;";
print  "git reset --hard ",$1," ; ";
print "cp -r /home/huan/gerrit/bash/* /home/huan/gerrit/utshell/";
print "cd /home/huan/gerrit/utshell/";
print "echo \"" $1 "\" >> record.txt";
print "git add .;"
print "git commit -m \"" , $2,"\"";
print "git push --set-upstream origin " $1;
print "git  checkout master; ";
print "cd /home/huan/gerrit/bash ";
print "sleep 10";
print "read"
}' ; done < ../1>./1.sh   
