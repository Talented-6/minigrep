#include <stdio.h>
int main()
{
    float x, y, a;
    printf("\033[01;31m");
    for (y = 1.5; y > -1.5; y -= 0.1)
    {
        if (y > -0.01 && y < 0.01)
        {
            printf("\033[0m");
            printf("\033[01;34m");
        }
        for (x = -1.5; x < 1.5; x += 0.05)
        {
            a = x * x + y * y - 1;
            putchar(a * a * a - x * x * y * y * y <= 0.0 ? '*' : ' ');
        }
        putchar('\n');
    }
    return 0;
}