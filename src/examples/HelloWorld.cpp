
#include <inttypes.h>
#include <iostream>
#include <stack>
#include <string>
#include <vector>

enum class ArgType
{
    bite,
    nom,
    chomp,
    drift,
    charms
};
        
union Arg
{
    const char* bite;
    const char* nom;
    const char* chomp;
    const char* drift;
    uint8_t charms;
};
        
struct ArgWrap
{
    ArgType t;
    std::vector<Arg> a;
};
        
int main()
{
    std::stack<ArgWrap> stack;
                    
    ArgWrap var1;
    var1.t = ArgType::charms;
                        
    Arg arg1;
    arg1.charms = 72;
    var1.a.push_back(arg1);
                        
    Arg arg2;
    arg2.charms = 101;
    var1.a.push_back(arg2);
                        
    Arg arg3;
    arg3.charms = 108;
    var1.a.push_back(arg3);
                        
    Arg arg4;
    arg4.charms = 108;
    var1.a.push_back(arg4);
                        
    Arg arg5;
    arg5.charms = 111;
    var1.a.push_back(arg5);
                        
    Arg arg6;
    arg6.charms = 44;
    var1.a.push_back(arg6);
                        
    Arg arg7;
    arg7.charms = 32;
    var1.a.push_back(arg7);
                        
    Arg arg8;
    arg8.charms = 119;
    var1.a.push_back(arg8);
                        
    Arg arg9;
    arg9.charms = 111;
    var1.a.push_back(arg9);
                        
    Arg arg10;
    arg10.charms = 114;
    var1.a.push_back(arg10);
                        
    Arg arg11;
    arg11.charms = 108;
    var1.a.push_back(arg11);
                        
    Arg arg12;
    arg12.charms = 100;
    var1.a.push_back(arg12);
                        
    Arg arg13;
    arg13.charms = 33;
    var1.a.push_back(arg13);
                        
    Arg arg14;
    arg14.charms = 10;
    var1.a.push_back(arg14);
                        
    stack.push(var1);
                        
    if (var1.t == ArgType::charms)
    {
        stack.pop();
        for (Arg a : var1.a)
        {
            printf("%c", a.charms);
        }
    }
                                
    return 0;
                        
}
                    