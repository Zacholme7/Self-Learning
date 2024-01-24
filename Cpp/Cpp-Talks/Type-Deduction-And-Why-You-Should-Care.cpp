// https://www.youtube.com/watch?v=wQxj20X-tIU
// this ties into the first part of effective modern cpp

// Notes
// c++98 type deduction is used only for templated
// c++11 scope expands, auto, universal ref, lambda captures, ...
// c++14 expanded even furthur

// general problem: need to deduce T and Paramtype from expr
// T is often different from Paramtype
// template <typename T>
// void f(Paramtype param)
// f(expr)

// three general casses
// 1) Paramtype is a reference or pointer, but not a universal reference
// - if expr's type is a reference, ignore that
// - pattern match exprs type against ParamType to determine T
// - auto plays the same role as T
// 2) Paramtype is a universal reference
// template<typename T>
// void f(T&& param);
// f(expr);
// - treated like normal ref papram expect if expr is lvalue with deduced type
// E, T is deduced as E&
// - only place in type deduction where T will be a refernce
// 3) Paramtype is neither a reference nor a pointer, by value
// - if exprs type is a reference, ignore that
// - is expr is const or volatile, ignore that
// - T is the result

// when using auto, it plays the role of T, ref/const is dropped
// auto is never deduced to be a reference, it must be manually added
// creates a brand new object unless it is adorned with references/pointers
// this is for c++ 14
// difference when you have braced initializer
// - a braced initializer has no type
// - this will fail type deduction
// - BUT, if you use auto a type will be deduced... an initilizer list
// after N3922, c++17
// using auto but dont use equal with initializer list
// - if one element, type of auto is the type of element inside

// lambda caputre type deduction
// three types
// 1) by reference: use template type deduction rules
// 2) c++14 init capture: uses auto type deduction
// 3) by values: use template type dedution except cv qualifiers are retained

// you can observe deducded types
// wont compile but in the error message you can see the type
template <typename T> class TD; // type displayer

template <typename T> // template with types of interest
void f(T &param) {
  TD<T> tType;
  TD<decltype(param)> paramType;
}

// for auto variables, use decltype to get type
// int x = 22;
// const int &rx = x;
// auto y = rx;
// TD<decltype(y)> yType; // compile diagnostics show type

// refer to lecture for deducing at runtime (47:00)
// avoid std::type_info::name

// decltype(name) = declared type of name
// never strips const/volatile references
int x = 10;        // decltype(x) = int
const auto &rx = x; // decltype(rx) = const int &

// function return type deduction
// makes code really confusing though.. be cautious
// c++14: extensive, all lambda + all functions
// auto: use template ( not auto ) type deduction rules
// decltype(auto): use decltype type deduction rules

// rules of thumb
// use auto if a reference type would never be correct
// use decltype(auto) only if a reference type could be correct






















