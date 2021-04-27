#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/wait.h>
#include <string.h>

int Fork()
{
    int rc = fork();
    if (rc < 0)
    {
        fprintf(stderr, "Fork failed\n");
        exit(1);
    }
    else
    {
        return rc;
    }
}

int Dup2(int old_fd, int new_fd)
{
    int rc = dup2(old_fd, new_fd);
    if (rc < 0)
    {
        fprintf(stderr, "Dup2 failed\n");
        exit(1);
    }
    else
    {
        return rc;
    }
}

int main(int argc, char const *argv[])
{
    int pipefd[2];
    if (pipe(pipefd) < 0)
    {
        fprintf(stderr, "Pipe failed\n");
        exit(1);
    }

    int out = dup(STDOUT_FILENO);
    Dup2(pipefd[1], STDOUT_FILENO);
    Dup2(pipefd[0], STDIN_FILENO);

    if (Fork() == 0)
    {
        printf("Message from process %d\n", getpid());
        return 0;
    }

    if (Fork() == 0)
    {
        char buf[100] = {0};

        scanf("%99[^\n]", buf);
        Dup2(out, STDOUT_FILENO);
        printf("%s, received by %d\n", buf, getpid());

        return 0;
    }

    for (int i = 0; i < 2; i++)
    {
        wait(NULL);
    }
}
