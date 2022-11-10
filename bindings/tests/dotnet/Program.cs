using System;

using RGML;

namespace RGMLTest
{
    internal class Program
    {
        static void Main(string[] args)
        {
            Vector4 t = Vector4.Identity;
            Matrix4x4 ident = Matrix4x4.Identity;

            Console.WriteLine(t * ident);

            Console.WriteLine("PASS!");
        }
    }
}
