<img align="right" alt="praise lord helix" src="http://assets.pokemon.com/assets/cms2/img/pokedex/full/139.png">

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

```
$ helix translate test.helix test.cpp
```

```cpp
#include <string>
#include <iostream>
using namespace std;

namespace fruits {

	struct banana {
		char name[50];int size;
	};

	struct guava {
		int intelligence;int size;
	};
}

int main() {
	fruits::banana my_banana;;
	fruits::guava my_guava;;
	printf("enter name of banana, please: ");
	cin.get(my_banana.name,50);
	printf("my banana's name is: %s",my_banana.name);
	return 0;
}
```
