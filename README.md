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
import "string" library

def test(int a)
	if a == 6
		return 2 + a
	else
		return 2 - a

	return a

def main()
	return test(5) # success

module something
	foo = "bar"

	module inside
		foo = "hey"
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
	if((6 == a)) {
		return (a + 2);
	} else {
		return (a - 2);
	}

	return a;
}

int main() {
	return test(5);
}

namespace something {
	string foo = "bar";
	
	namespace inside {
		string foo = "hey";
	}
}
```
