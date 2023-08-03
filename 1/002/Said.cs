static int FibonacciSum(int limit)
    {
        int a = 1, b = 2, total = 0;
        while (a <= limit)
        {
            if (a % 2 == 0)
                total += a;
            int temp = a;
            a = b;
            b = temp + b;
        }
        return total;
    }
