#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/wait.h>

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

int main(int argc, char const *argv[])
{
    if (Fork() == 0)
    {
        printf("execl\n");
        // The arguemnt list itself must be terminated by a null pointer. How is this even
        // acceptable as a sane API?
        execl("/bin/ls", "/bin/ls", "-l", NULL);
    }

    wait(NULL);

    if (Fork() == 0)
    {
        printf("execle\n");
        char *const envp[] = {"TIME_STYLE=long-iso", NULL};
        execle("/bin/ls", "/bin/ls", "-l", NULL, envp);
    }

    wait(NULL);

    if (Fork() == 0)
    {
        printf("execlp\n");
        execlp("ls", "ls", "-l", NULL);
    }

    wait(NULL);

    if (Fork() == 0)
    {
        printf("execv\n");
        char *const argv[] = {
            "/bin/ls",
            "-l",
            NULL};

        execv("/bin/ls", argv);
    }

    wait(NULL);

    if (Fork() == 0)
    {
        printf("execvp\n");
        char *const argv[] = {
            "/bin/ls",
            "-l",
            NULL};

        execvp("ls", argv);
    }

    if (Fork() == 0)
    {
        printf("execvpe\n");

        char *const argv[] = {
            "/bin/ls",
            "-l",
            NULL};

        char *const envp[] = {"TIME_STYLE=long-iso", NULL};

        execvpe("ls", argv, envp);
    }

    wait(NULL);
}