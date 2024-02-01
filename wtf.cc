int wtf(int* x, int* y)
{
    *x = 42;
    *y = 99;
    return *x;
}

int main()
{
    int x=0;
    return wtf(&x, &x);
}
