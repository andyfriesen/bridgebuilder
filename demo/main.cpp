#include <stdio.h>

#include "output.h"

extern "C" {
    const Foo* make_Foo(int variant, int value);
    void drop_Foo(const Foo* foo);
}

void printFoo(const Foo* foo) {
    if (auto nil = get<Foo::Nil>(foo))
        printf("Nil");
    else if (auto boolean = get<Foo::Boolean>(foo))
        printf("Boolean(%s)", (*boolean) ? "true" : "false");
    else if (auto integer = get<Foo::Integer>(foo))
        printf("Integer(%d)", *integer);
    else
        printf("???");
    printf("\n");
}

int main() {
    const Foo* f = make_Foo(1, 1);

    printFoo(f);

    drop_Foo(f);

    printf("Hello world!\n");
}
