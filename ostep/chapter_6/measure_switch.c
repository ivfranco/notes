#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>
#include <sched.h>
#include <sys/wait.h>
#include <sched.h>

#define REPEAT 100000

void Pipe(int pipefd[2])
{
    if (pipe(pipefd) < 0)
    {
        fprintf(stderr, "Pipe creation failed\n");
        exit(1);
    }
}

void set_current_process_affinity(int cpu)
{
    cpu_set_t set;
    CPU_ZERO(&set);
    CPU_SET(cpu, &set);

    if (sched_setaffinity(getpid(), sizeof(set), &set) < 0)
    {
        fprintf(stderr, "Set affinity failed\n");
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
        set_current_process_affinity(0);

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

        set_current_process_affinity(0);
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
