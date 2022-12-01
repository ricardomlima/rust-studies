/* SCALAR TYPES

represent a single value

INTEGERS

Length	Signed	Unsigned
8-bit	i8	u8
16-bit	i16	u16
32-bit	i32	u32
64-bit	i64	u64
128-bit	i128	u128
arch	isize	usize

*/
let x: usize = 16;

/* FLOATING POINTS

types available: f32 & 64

*/
let floater32: f32 = 2.0;
let floater64: f64 = 3.0;

/* NUMERIC OPERATIONS */

// addition
let sum = 5 + 10;

// subtraction
let difference = 95.5 - 4.3;

// multiplication
let product = 4 * 30;

// division
let quotient = 56.7 / 32.2;
let floored = 2 / 3; // Results in 0

// remainder
let remainder = 43 % 5;

/* BOOLEAN TYPE */
let t = true; // implicit typing

let f: bool = false; // with explicit type annotation

/* CHARACTER TYPE */

// most primitive
let c = 'z';
let z: char = 'ℤ'; // with explicit type annotation
let heart_eyed_cat = '😻';
