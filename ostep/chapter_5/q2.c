#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include <string.h>
#include <sys/wait.h>
#include <errno.h>

extern int errno;

int main(int argc, char const *argv[])
{
    int fd = open("./tmp", O_CREAT | O_TRUNC | O_RDWR);
    if (fd < 0)
    {
        fprintf(stderr, "Open failed\n");
        exit(1);
    }

    int rc = fork();
    const size_t SIZE = 1000;

    if (rc < 0)
    {
        fprintf(stderr, "Fork failed\n");
        exit(1);
    }
    else if (rc == 0)
    {
        // child process
        char buf[100] = {0};
        memset(buf, 'A', 100);
        write(fd, buf, 100);
    }
    else
    {
        // parent process
        char buf[201] = {0};
        memset(buf, 'B', 100);
        write(fd, buf, 100);

        wait(NULL);

        lseek(fd, 0, SEEK_SET);
        printf("file size: %d\n", read(fd, buf, 200));
        printf("%s", buf);
    }
}
