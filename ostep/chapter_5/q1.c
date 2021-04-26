#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int main(int argc, char *argv[])
{
    int x = 100;

    int rc = fork();

    if (rc < 0)
    {
        fprintf(stderr, "Fork failed\n");
        exit(1);
    }
    else if (rc == 0)
    {
        // child process
        printf("child process: x = %d\n", x);
        x = 200;
        printf("child process: x = %d\n", x);
    }
    else
    {
        // parent proces
        printf("parent process: x = %d\n", x);
        x = 300;
        printf("parent process: x = %d\n", x);
    }
}