#include <stdlib.h>

int main(int argc, char const *argv[])
{
    int *data = malloc(100 * sizeof(int));
    free(data + 10);
}
