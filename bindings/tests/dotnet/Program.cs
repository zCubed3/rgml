using System;

using PrismMath;

namespace PrismMathTest
{
    internal class Program
    {
        static void Main(string[] args)
        {
            Vector3 vA = new Vector3(0, 0, 0);
            Vector3 vB = new Vector3(1, 1, 1);

            Vector3 v3 = vA + vB;

            Console.WriteLine($"{v3[0]}, {v3[1]}, {v3[2]}");
        }
    }
}
