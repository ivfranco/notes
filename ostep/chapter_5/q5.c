#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/wait.h>
#include <errno.h>
#include <string.h>

int main(int argc, char const *argv[])
{
    int rc = fork();

    if (rc < 0)
    {
        fprintf(stderr, "Fork failed\n");
        exit(1);
    }
    else if (rc == 0)
    {
        // child process
        wait(NULL);
        fprintf(stderr, "%s\n", strerror(errno));
    }
    else
    {
        // parent process
        pid_t pid = wait(NULL);
        printf("child %d terminated\n", pid);
    }
}
