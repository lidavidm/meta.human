use std::fmt;
use std::ops;

macro_rules! isa_type_op {
    ($name: ident, $ty: ty, $op: ident, $op_name: ident) => {
        impl ops::$op<$name> for $name {
            type Output = $name;

            fn $op_name(self, _rhs: $name) -> $name {
                $name(ops::$op::$op_name(self.0, _rhs.0))
            }
        }

        impl ops::$op<$ty> for $name {
            type Output = $name;

            fn $op_name(self, _rhs: $ty) -> $name {
                $name(ops::$op::$op_name(self.0, _rhs))
            }
        }
    }
}

macro_rules! isa_type_assign_op {
    ($name: ident, $ty: ty, $op: ident, $op_name: ident) => {
        impl ops::$op<$name> for $name {
            fn $op_name(&mut self, _rhs: $name) {
                ops::$op::$op_name(&mut self.0, _rhs.0)
            }
        }

        impl ops::$op<$ty> for $name {
            fn $op_name(&mut self, _rhs: $ty) {
                ops::$op::$op_name(&mut self.0, _rhs)
            }
        }
    }
}

macro_rules! isa_type {
    ($name: ident, $utype: ty) => {
        #[derive(Clone,Copy,Debug,Eq,Hash,Ord,PartialEq,PartialOrd)]
        pub struct $name(pub $utype);

        impl $name {
            pub fn wrapping_add(self, rhs: Self) -> Self {
                $name(self.0.wrapping_add(rhs.0))
            }

            pub fn wrapping_sub(self, rhs: Self) -> Self {
                $name(self.0.wrapping_sub(rhs.0))
            }

        }

        isa_type_op!($name, $utype, Add, add);
        isa_type_assign_op!($name, $utype, AddAssign, add_assign);
        isa_type_op!($name, $utype, Sub, sub);
        isa_type_op!($name, $utype, Mul, mul);
        isa_type_op!($name, $utype, Div, div);
        isa_type_op!($name, $utype, Rem, rem);
        isa_type_op!($name, $utype, Shr, shr);
        isa_type_op!($name, $utype, Shl, shl);
        isa_type_op!($name, $utype, BitAnd, bitand);
        isa_type_op!($name, $utype, BitOr, bitor);
        isa_type_op!($name, $utype, BitXor, bitxor);

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                write!(f, "{}", self.0)
            }
        }

        impl fmt::LowerHex for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                write!(f, "{:x}", self.0)
            }
        }

        impl fmt::UpperHex for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                write!(f, "{:X}", self.0)
            }
        }
    }
}

pub trait IsaType {
    type Unsigned;
    type Signed;

    fn as_signed(self) -> Self::Signed;
    fn as_signed_word(self) -> SignedWord;
    fn as_word(self) -> Word;
    fn as_halfword(self) -> HalfWord;
    fn as_byte(self) -> Byte;
    fn as_address(self) -> Address;

    /// Converts the type into bytes, LSB-first.
    fn as_bytes(self) -> Vec<Byte>;
}

macro_rules! isa_utype {
    ($name: ident, $signed: ident, $utype: ty, $stype: ty) => {
        impl IsaType for $name {
            type Unsigned = $name;
            type Signed = $signed;

            fn as_signed(self) -> Self::Signed {
                $signed(self.0 as $stype)
            }

            fn as_signed_word(self) -> SignedWord {
                // Convert self to signed so that second cast will
                // sign-extend
                SignedWord((self.0 as $stype) as i32)
            }

            fn as_word(self) -> Word {
                Word(self.0 as u32)
            }

            fn as_halfword(self) -> HalfWord {
                HalfWord(self.0 as u16)
            }

            fn as_byte(self) -> Byte {
                Byte(self.0 as u8)
            }

            fn as_address(self) -> Address {
                self.as_word()
            }

            fn as_bytes(self) -> Vec<Byte> {
                use std::mem;

                let mut bytes = vec![];
                for offset in 0..mem::size_of::<$utype>() {
                    bytes.push(Byte(((self.0 >> (8 * offset)) & 0xFF) as u8));
                }

                bytes
            }
        }

        impl IsaType for $signed {
            type Unsigned = $name;
            type Signed = $signed;

            fn as_signed(self) -> Self::Signed {
                self
            }

            fn as_signed_word(self) -> SignedWord {
                SignedWord(self.0 as i32)
            }

            fn as_word(self) -> Word {
                Word(self.0 as u32)
            }

            fn as_halfword(self) -> HalfWord {
                HalfWord(self.0 as u16)
            }

            fn as_byte(self) -> Byte {
                Byte(self.0 as u8)
            }

            fn as_address(self) -> Address {
                self.as_word()
            }

            fn as_bytes(self) -> Vec<Byte> {
                use std::mem;

                let mut bytes = vec![];
                for offset in 0..mem::size_of::<$utype>() {
                    bytes.push(Byte((self.0 >> (8 * offset)) as u8));
                }

                bytes
            }
        }
    }
}

isa_type!(Word, u32);
isa_type!(SignedWord, i32);
isa_utype!(Word, SignedWord, u32, i32);
isa_type!(HalfWord, u16);
isa_type!(SignedHalfWord, i16);
isa_utype!(HalfWord, SignedHalfWord, u16, i16);
isa_type!(Byte, u8);
isa_type!(SignedByte, i8);
isa_utype!(Byte, SignedByte, u8, i8);

pub type Address = Word;
