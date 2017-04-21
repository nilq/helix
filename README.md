<img align="right" width="35%" height="35%" alt="praise lord helix" src="http://assets.pokemon.com/assets/cms2/img/pokedex/full/139.png">

# helix
A programming language transpiler

```
helix language

usage:
    helix run <source>
    helix build <source> <destination>
    helix translate <source> <destination>
    helix (-h | --help)
    helix --version

options:
    -h --help   display this message
    --version   display version
```

## Examples

test.helix
```helix
import "iostream" library

class dog
  function hello (name: int, age: int)

class bird <- dog
  function hello (name: int, age: int) -> int

implement bird
  function hello (name: int, age: int)
    printf("hello, %i, age %i", name, age)
    return 1

module bar
  function foo (a: int, b: int)
    var a1 = 100

    if a + b == 0
      return a1

    return a + b

function main
  bob: bird
  bob.hello(1, 2)

  return bar::foo(1, -1)
```

```
$ helix translate test.helix test
```

test.hpp
```cpp
#ifndef test
#define test
#include <iostream>

class dog {
public:
	void hello (int name,int age);
};
class bird : dog {
public:
	int hello (int name,int age);
};
#endif
```

test.cpp
```cpp
#include "test.hpp"

int bird::hello(int name,int age) {
	printf("hello, %i, age %i",name,age);
return 1;
;
}
namespace bar {
	auto foo(int a,int b) {
	int a1 = 100;;
if((a + (b == 0))) {return a1;
};
return (a + b);
;
}
}int main() {
	bird bob;;
bob.hello(1,2);
return bar::foo(1,-1);
;
}
```
