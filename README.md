![praise lord helix](http://assets.pokemon.com/assets/cms2/img/pokedex/full/139.png)

# helix
A programming language transpiler

```
helix language

usage:
    helix repl
    helix translate <source>
    helix (-h | --help)
    helix --version

options:
    -h --help   display this message
    --version   display version
```

## Examples

```helix
# test.helix
import "iostream" library
import "string"   library

def test(a)
    return 3 + a

def main()
	return test(6)

module something
	module test
		foo = "bar"

	module another_test?
		foo = "another bar"

	if true == false
		a = 123 + 2

	a = 1

	if false == false
		a = 1

	module testtt
		ayy = "1"
```

```
$ helix translate test.helix test.cpp
```

```cpp
// test.cpp
#include <iostream>
#include <string>

using namespace std;

auto test(int a) {
    return (a + 3);
}

int main() {
    return test(5);
}

namespace something {
	namespace test {
	    string foo = "bar";
    }
    namespace another_test? {
        string foo = "another bar";
    }
    
    int a = 1;

    namespace testtt {
        string ayy = "1";
    }
}
```
