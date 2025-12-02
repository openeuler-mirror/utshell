一、第一轮修改
1、goto 的match形式 看看是否能改成loop形式，match形式不一定是不可以，能否把数字改成字符
2、有些宏被写成了函数，这部分是否需要修改（fc.rs）
3、signal.rs比较特殊，目前还不清楚作用是什么,wait.rs中使用
4、builtins下面有几个文件名和rust关键字重名，如何修改
5、expr.rs中的sh_free?
6、main.rs中的main函数修改
7、y_tab 没有引用rbash，不用修改？

8、r_instruction_r_output_direction: r_instruction = 0;中的r_instruction_r_output_direction去掉r_instruction_前缀




二、第二轮修改特征
1、类型不匹配，对于函数指针有多余的“Option”描述


三、库和重构函数出现函数重复定义问题分析
出现重复定义的函数，均与readline/shell.c中重复

1、sh_set_lines_and_columns函数
    1）函数分别定义在：variables.rs和readline/shell.c中
    2）variables中的该函数没有找到调用的地方
    3）readline/shell.c中的该函数只在readline文件中使用到了
    4）建议：将variables中的函数改为另外一个名字
    
　　//bgz:bash里面，不是同一个函数。重命名src/variables.rs下的sh_set_lines_and_columns函数,rename sh_set_line_and_columns as sh_set_line_and_columns_out;

2、sh_get_env_value函数
    1）函数实现不同,函数分别定义在：variables.rs和readline/shell.c中
    2）在variables中有如下注释：“/* This is present for use by the tilde and readline libraries. */，经过查找发现
       tilde和readline库使用的是readline/shell.c中的函数

    //bgz:将文件src/variables.rs文件中相应函数重命名为sh_get_env_value_rename();

3、sh_get_home_dir
    
    //bgz:variables.rs和shell.c中经查看，应该是一样的(c中使用ifdefine较多)，处理方式：注释rs文件中此函数,并于开头处添加对c.a的调用(意为调用c.a中的同名函数)；

4、sh_single_quote
    1)函数分别定义在：sh/shquote.c和readline/shell.c中
    2)函数逻辑看着差不多，还没仔细看
    3）除sh/shquote.c中使用的函数是本文件中的，其他位置调用的都是readline/shell.c中的该函数
    4）建议：如果函数逻辑相同，则在sh/shquote.c中的函数前面加static

    //bgz:注释shquote.c中sh_single_quote函数,更新c.a 。

5、sh_unset_nodelay_mode

    //bgz:处理方式同3。

6、src_common.rs和src/bin/utshellversion.rs中关于shell_name变量重复定义，我将utshellversion.rs中变量前面的pub去除，错误消除；

    //bgz:将src/bin/utshellversion.rs中变量shell_name重命名为shell_name_rename。


