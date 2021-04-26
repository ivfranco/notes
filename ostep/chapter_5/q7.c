#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

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
        close(STDOUT_FILENO);
        int size = printf("stdout closed\n");
        fprintf(stderr, "printf returned %d\n", size);
    }
}
