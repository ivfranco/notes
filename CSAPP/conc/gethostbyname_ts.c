#include <stdio.h>
#include <stdlib.h>
#include <netdb.h>
#include <sys/socket.h>
#include <arpa/inet.h>
#include <semaphore.h>
#include <string.h>

static sem_t mutex;

void init();
size_t array_len(char **);
char **deep_copy_strs(char **);
struct hostent *gethostbyname_ts(char *);

int main(int argc, char *argv[]) {
    struct hostent *hostp;
    struct in_addr addr;
    char **pp, *name;

    init();
    name = argv[1];
    hostp = gethostbyname_ts(name);
    for (pp = hostp->h_addr_list; *pp != NULL; pp++) {
        addr.s_addr = ((struct in_addr *)*pp) -> s_addr;
        printf("address: %s\n", inet_ntoa(addr));
    }

    return 0;
}

void init() {
    sem_init(&mutex, 0, 1);
}

size_t array_len(char **ptr) {
    size_t cnt = 0;
    while (*ptr != NULL) {
        ptr++;
        cnt++;
    }
    return cnt;
}

char **deep_copy_strs(char **ss) {
    char **cpy = (char **)malloc((array_len(ss) + 1) * sizeof(char *));
    char **cpy_iter = cpy;

    while (*ss != NULL) {
        *cpy_iter = strdup(*ss);
        ss++;
        cpy_iter++;
    }
    *cpy_iter = NULL;
    return cpy;
}

struct hostent *gethostbyname_ts(char *name) {
    struct hostent *hostp, *hostp_s;
    
    // printf("acquiring semaphore\n");
    sem_wait(&mutex);
    // printf("acquired semaphore\n");
    hostp = (struct hostent *)malloc(sizeof(struct hostent));
    hostp_s = gethostbyname(name);
    // printf("gethostbyname returned\n");
    hostp->h_addrtype = hostp_s->h_addrtype;
    hostp->h_length = hostp_s->h_length;
    hostp->h_name = strdup(hostp_s->h_name);
    hostp->h_aliases = deep_copy_strs(hostp_s->h_aliases);
    hostp->h_addr_list = deep_copy_strs(hostp_s->h_addr_list);
    // printf("releasing semaphore\n");
    sem_post(&mutex);
    // printf("released semaphore\n");
    return hostp;
}
