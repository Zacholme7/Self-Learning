#include <iostream>
#include <sstream>
#include <string>
#include <vector>

// variadic templates are a way to pass multiple parameters into a generic template function
// templates can have compile time and runtime overhead
// especially when doing them in a recursive way

// this will print anything
// assumes that T has a ostream operator overload
// there will be a concrete time for anything passed in
template<typename T>
void print(const T& t) {
    std::cout << t << '\n';
}

// return string representation of any value
template<typename T>
std::string toString(const T& t) {
    std::stringstream ss;
    ss << t;
    return ss.str();
}

// variadic template
// ... signals variadic
// can do this recursively by peeling off params but this generates a lot of boiler plate code, takes long to compile, and can get out of hand
template<typename ... Param>
std::vector<std::string> toStringVariadic(const Param& ... param) {

    // could also use generic lambda
    //   const auto toString = [](const auto &t) {
    //     std::stringstream s;
    //     ss << t;
    //     return ss.str()
    //   }
    //
    // use initializer list to init vector of strings
    // will expand all the calls for the toString calls for each param
    return {toString(param)...};
} 

int main() {
    // print template
    print(1);
    print("hello");
    print(5.3);

    // toString template
    toString(1);
    toString("hello");
    toString(5.3);

    // variadic template
    const auto vec = toStringVariadic("hello", 1, 5.3);
    for(const auto &o: vec) {
        std::cout << o << '\n';
    }
    return 0;
}
