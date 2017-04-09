#include <iostream>
#include <string>
using namespace std;
auto test(int a) {
	if((6 == a)) {return (a + 2);
} else {return (a - 2);
}}int main() {
	return test(5);
}namespace something {
	string foo = "bar";namespace inside {
	string foo = "hey";
}
}