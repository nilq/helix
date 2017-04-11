#include <string>
#include <iostream>
using namespace std;
namespace fruits {
	struct banana {
	char name[50];int size;
};struct guava {
	int intelligense;int size;
};
}int main() {
	fruits::banana my_banana;;
fruits::guava my_guava;;
printf("enter name of banana, please: ");
cin.get(my_banana.name,50);
printf("my banana's name is: %s",my_banana.name);
return 0;
;
}