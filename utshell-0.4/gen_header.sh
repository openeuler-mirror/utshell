mkdir -p u_src/src/header/
rm -f u_src/src/header/*.rs
for head in `ls -1 *.h`
do
    filename=`basename -s .h $head`
    bindgen $head -o u_src/src/header/${filename}_h.rs --no-layout-tests --use-core -- -DSELECT_COMMAND \
    -DARITH_FOR_COMMAND -DCOND_COMMAND -DDPAREN_ARITHMETIC \
    -I include -I/usr/include -include unistd.h \
    -include stdint.h -include include/stdc.h -include command.h \
    -include general.h -include array.h -include trap.h \
    -include variables.h -include config.h -include externs.h

done
:<<BLOCK
    bindgen $head -o u_src/src/header/${filename}_h.rs --no-layout-tests --use-core -- -DSELECT_COMMAND \
    -DARITH_FOR_COMMAND -DCOND_COMMAND -DDPAREN_ARITHMETIC \
    -I include -I/usr/include -include unistd.h \
    -include stdint.h -include include/stdc.h -include command.h \
    -include general.h -include array.h -include trap.h \
    -include variables.h -include config.h -include externs.h
BLOCK