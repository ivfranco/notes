#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>

#define REPEAT 1000000

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
    struct timespec before;
    struct timespec after;

    clock_gettime(CLOCK_MONOTONIC, &before);

    for (int i = 0; i < REPEAT; i++)
    {
        getpid();
    }

    clock_gettime(CLOCK_MONOTONIC, &after);

    printf("%llu nano seconds\n", time_diff(before, after) / REPEAT);
}
