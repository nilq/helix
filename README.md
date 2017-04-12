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

import "localfile.h"

def test(int a)
	if a == 6
		return 2 + a
	else
		return 2 - a

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
#include "localfile.h"

using namespace std;

auto test(int a) {
	if((6 == a)) {
		return (a + 2);
	} else {
		return (a - 2);
	}
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

## More examples

```helix
import "string"   library
import "iostream" library

module fruits
	struct banana
		name[50]: char
		size: int

	struct guava
		intelligence: int
		size: int

def main()

	my_banana: fruits::banana
	my_guava:  fruits::guava

	printf("enter name of banana, please: ")

	cin.get(my_banana.name, 50)

	printf("my banana's name is: %s", my_banana.name)

	return 0
```
