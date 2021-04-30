#include <stdlib.h>

int main(int argc, char const *argv[])
{
    int *data = malloc(100 * sizeof(int));
    data[100] = 0;
    free(data);
}
