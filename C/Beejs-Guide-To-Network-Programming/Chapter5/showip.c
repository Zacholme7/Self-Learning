#include <stdio.h>
#include <string.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netdb.h>
#include <arpa/inet.h>
#include <netinet/in.h>

int main(int argc, char *argv[]) {
    // hints for getaddrinfo and res to put the result in
    struct addrinfo hints, *res, *p;
    int status;
    char ipstr[INET6_ADDRSTRLEN];

    if (argc != 2) {
        fprintf(stderr, "usage: showip hostname\n");
        return 1;
    }

    // usage of getaddrinfo
    memset(&hints, 0, sizeof hints); // zero out hints
    hints.ai_family = AF_UNSPEC; // make it use both ipv4 and 6
    hints.ai_socktype = SOCK_STREAM; // set to tcp

    // fill in our addrinfo res
    if ((status = getaddrinfo(argv[1], NULL, &hints, &res)) != 0) {
        fprintf(stderr, "getaddrinfo: %s\n", gai_strerror(status));
        return 2;
    }

    printf("IP addresses for %s:\n\n", argv[1]);

    // loop through the linked list
    for( p = res; p != NULL; p = p->ai_next) {
        void *addr;
        char *ipver;

        if(p->ai_family == AF_INET) { // IPV4
            // cast to a ipv4 sockaddr_in
            struct sockaddr_in *ipv4 = (struct sockaddr_in *)p->ai_addr;
            addr = &(ipv4->sin_addr);
            ipver = "IPv4";
        } else {
            // cast to a ipv6 sockaddr_in
            struct sockaddr_in6 *ipv6 = (struct sockaddr_in6 *)p->ai_addr;
            addr = &(ipv6->sin6_addr);
            ipver = "IPv6";
        }

        // conver the ip to a string and print it
        inet_ntop(p->ai_family, addr, ipstr, sizeof ipstr);
        printf("  %s: %s\n", ipver, ipstr);
    }

    freeaddrinfo(res); // free the linked list
    return 0;
}
