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

```helix
# test.helix
class bird
  function hello (name: int, age: int)

implement bird
  function hello (name: int, age: int)
    printf("hello, %i, age %i", name, age)

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

class bird {
public:
	void hello (int name,int age);
};
#endif
```

test.cpp
```cpp
#include "test.hpp"

void bird::hello(int name,int age) {
	printf("hello, %i, age %i",name,age);
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
