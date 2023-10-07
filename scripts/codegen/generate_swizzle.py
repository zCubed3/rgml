#!/bin/python

# Generates a swizzle macro block for use in the rgml math library
# The resulting file is swizzle.gen.rs
import itertools

indices_to_comp = {
    0: ["x", "u", "s", "r"],
    1: ["y", "v", "t", "g"],
    2: ["z", "", "q", "b"],
    3: ["w", "", "", "a"],
}

macro_block = \
"""
use crate::vector::Vector;
use crate::real::*;

macro_rules! vector_swizzle_single {
    ($count:literal, $index:literal, $c:ident) => {
        impl<T: RealNumber> Vector<T, $count> {
            #[doc(hidden)]
            #[inline(always)]
            pub fn $c(&self) -> T {
                return self[$index];
            }
        }
    };
}

macro_rules! vector_swizzle_double {
    ($count:literal, $index:literal, $index2:literal, $cc:ident) => {
        impl<T: RealNumber> Vector<T, $count> {
            #[doc(hidden)]
            #[inline(always)]
            pub fn $cc(&self) -> Vector<T, 2> {
                return Vector::<T, 2>::new(self[$index], self[$index2]);
            }
        }
    };
}

macro_rules! vector_swizzle_triple {
    ($count:literal, $index:literal, $index2:literal, $index3:literal, $ccc:ident) => {
        impl<T: RealNumber> Vector<T, $count> {
            #[doc(hidden)]
            #[inline(always)]
            pub fn $ccc(&self) -> Vector<T, 3> {
                return Vector::<T, 3>::new(self[$index], self[$index2], self[$index3]);
            }
        }
    };
}

macro_rules! vector_swizzle_quadruple {
    ($count:literal, $index:literal, $index2:literal, $index3:literal, $index4:literal, $cccc:ident) => {
        impl<T: RealNumber> Vector<T, $count> {
            #[doc(hidden)]
            #[inline(always)]
            pub fn $cccc(&self) -> Vector<T, 4> {
                return Vector::<T, 4>::new(self[$index], self[$index2], self[$index3], self[$index4]);
            }
        }
    };
}\n
"""


def gen_swizzle(file, indices: [int]):
    length = len(indices)
    indice_count = len(indices_to_comp[0])

    for i in indices:
        chars = indices_to_comp[i]

        for char in chars:
            if char != "":
                file.write(f"vector_swizzle_single!(%i, %i, %s);\n" % (length, i, char))

    for two in list(itertools.product(indices, repeat=2)):
        pairs = list(zip(indices_to_comp[two[0]], indices_to_comp[two[1]]))

        for pair in pairs:
            if any(comp == "" for comp in pair):
                continue

            file.write(f"vector_swizzle_double!(%i, %i, %i, %s);\n" % (length, two[0], two[1], "".join(pair)))

    for three in list(itertools.product(indices, repeat=3)):
        pairs = list(zip(indices_to_comp[three[0]], indices_to_comp[three[1]], indices_to_comp[three[2]]))

        for pair in pairs:
            if any(comp == "" for comp in pair):
                continue

            file.write(f"vector_swizzle_triple!(%i, %i, %i, %i, %s);\n" % (length, three[0], three[1], three[2], "".join(pair)))

    for quad in list(itertools.product(indices, repeat=4)):
        pairs = list(zip(indices_to_comp[quad[0]], indices_to_comp[quad[1]], indices_to_comp[quad[2]], indices_to_comp[quad[3]]))

        for pair in pairs:
            if any(comp == "" for comp in pair):
                continue

            file.write(f"vector_swizzle_quadruple!(%i, %i, %i, %i, %i, %s);\n" % (length, quad[0], quad[1], quad[2], quad[3], "".join(pair)))


with open("src/vector/swizzle.rs", "w") as file:
    file.write("//\n")
    file.write("// Generated automatically by generate_swizzle.py\n")
    file.write("//\n\n")

    file.write("// SWIZZLE MACRO BLOCK\n")
    file.write(macro_block)
    file.write("// -------------------\n\n")

    file.write("// VECTOR2 SWIZZLE BLOCK\n")
    gen_swizzle(file, [0, 1])
    file.write("// ---------------------\n\n")

    file.write("// VECTOR3 SWIZZLE BLOCK\n")
    gen_swizzle(file, [0, 1, 2])
    file.write("// ---------------------\n\n")

    file.write("// VECTOR4 SWIZZLE BLOCK\n")
    gen_swizzle(file, [0, 1, 2, 3])
    file.write("// ---------------------\n\n")