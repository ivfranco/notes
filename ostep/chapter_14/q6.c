#include <stdio.h>
#include <stdlib.h>

int main(int argc, char const *argv[])
{
    int *data = malloc(100 * sizeof(int));
    free(data);

    printf("%d", data[0]);
}
