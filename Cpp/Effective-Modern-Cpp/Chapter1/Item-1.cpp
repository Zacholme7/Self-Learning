#include <iostream>

// #1
// think of function template as looking like this
// template<typename T>
// void f(ParamType param)

// Case 1
template<typename T>
void f(T& param);

template<typename T>
void z(T* param);

// Case 2
template<typename T>
void a(T&& param);

// Case 3
template<typename T>
void b(T param);


int main() {
        // #1
        // call to a funciton template
        // f(expr) -> call to f with some expression

        // Case 1
        int x = 27; // x is an int
        const int cx = x; // cs is a const int
        const int &rx = x; // rx is a reference to x as a const int
        f(x); // T is an int, params type is int&
        f(cx); // T is a const int, params type is const int&
        f(rx); // T is a const int, params type is a const int&

        const int *px = &x; // px is a ptr to x as a const int
        z(&x); // T is a int, param is int*
        z(px); // T is a const int, param is const int*

        // Case 2
        a(x); // x is a lvalue, so T is a int& and paramtype is an int&
        a(cx); // cx is a lvalue, so T is a const int& and paramtype is const int&
        a(rx); // rx is a lvalue, so T is const int& and paramtype is const int&
        a(27); // 27 is a rvalue, so T is an int and paramtype is a int&&

        // Case 3
        // the value is copied in
        f(x); // T is a int, paramtype is an int
        f(cx); // T is an int, paramtype is an int
        f(rx); //  T is an int, paramtype is an int


        return 0;
}