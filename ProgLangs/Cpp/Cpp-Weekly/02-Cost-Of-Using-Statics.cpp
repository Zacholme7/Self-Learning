// Notes
// ------------
// construct on first use: technique used to manage the initialization of static variables
// used to solve the "static initialization order fiasco"
// problem where the order of initialization of static variables across different translation units is undefined
// how it works
// 1) instead of having global static object, provide function that returns ref to static local var 
// constructed when the funciton is called (lazy initialization)
// 2) c++ standard guarantees that local static variables are initialized hte first time control passes through their declarations
// obj is fully initialized the first time you need to use it
// 3) helps void issues related to the destruction of static objects, destoryed in reverse order of creation
// video didnt have much of substance, talked about some overhead of accessing static variables

// example of construct on first use
class MyObject{
public:
        MyObject() { /* constructor code*/ }
        void doSomething() { /* method impl */}
};

MyObject& getMyObject() {
        static MyObject instance; // constructed on first use
        return instance;
}

int main() {
        getMyObject().doSomething(); // the instance is created here on the first call
        return 0;
}
