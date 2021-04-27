#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>
#include <sched.h>
#include <sys/wait.h>

#define REPEAT 100000

void Pipe(int pipefd[2])
{
    if (pipe(pipefd) < 0)
    {
        fprintf(stderr, "Pipe creation failed\n");
        exit(1);
    }
}

long long int time_diff(
    struct timespec before,
    struct timespec after)
{
    long long int diff = 0;

    diff += (after.tv_sec - before.tv_sec) * 1000llu * 1000llu * 1000llu;
    diff += after.tv_nsec - before.tv_nsec;

    return diff;
}

int main(int argc, char const *argv[])
{
    int parent_to_child[2];
    int child_to_parent[2];

    Pipe(parent_to_child);
    Pipe(child_to_parent);

    int rc = fork();

    if (rc < 0)
    {
        fprintf(stderr, "Fork failed\n");
        exit(1);
    }
    else if (rc == 0)
    {
        // child process
        for (int i = 0; i < REPEAT; i++)
        {
            char buf[1] = {0};
            write(child_to_parent[1], buf, 1);
            read(parent_to_child[0], buf, 1);
        }

        return 0;
    }
    else
    {
        // parent process
        struct timespec before;
        struct timespec after;

        clock_gettime(CLOCK_MONOTONIC, &before);

        for (int i = 0; i < REPEAT; i++)
        {
            char buf[1] = {0};
            read(child_to_parent[0], buf, 1);
            write(parent_to_child[1], buf, 1);
        }

        wait(NULL);

        clock_gettime(CLOCK_MONOTONIC, &after);

        printf("%lld nanoseconds\n", time_diff(before, after) / (REPEAT * 2llu));
    }
}
