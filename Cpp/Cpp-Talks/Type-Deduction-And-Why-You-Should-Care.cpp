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
// 2) Paramtype is a universal reference
// 3) Paramtype is neither a reference nor a pointer
