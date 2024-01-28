// 23: c++17s has_include

// just look if we have access and if we do, include it
// this is good for cross platform and making sure we have the correct includes
#if __has_include(<unistd.h>)
#include <unistd.h>
#endif

int main() {
}

