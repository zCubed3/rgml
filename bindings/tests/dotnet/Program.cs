using System;

using PrismMath;

namespace PrismMathTest
{
    internal class Program
    {
        static void Main(string[] args)
        {
            Vector3 vA = new Vector3(0, 0, 1);
            Vector3 vB = new Vector3(0, 0, 1);

            // Calls all possible methods for Vector3
            _ = vA + vB;
            _ = vA - vB;
            _ = vA * vB;
            _ = vA / vB;

            _ = vA.Dot(vB);
            _ = vA.Min(vB);
            _ = vA.Max(vB);
            _ = vA.Cross(vB);

            _ = vA.Abs();
            _ = vA.Sin();
            _ = vA.Cos();
            _ = vA.Tan();
            _ = vA.Sum();

            Console.WriteLine("PASS!");
        }
    }
}
