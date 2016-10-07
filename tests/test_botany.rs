extern crate emperust;

use emperust::botany::{byte_to_bitarray, bitarray_to_byte};

#[test]
fn bitarray_conversion() {
    use emperust::botany::{byte_to_bitarray, bitarray_to_byte};
    assert_eq!(0b10010110, bitarray_to_byte([true, false, false, true,
                                             false, true, true, false]));
    assert_eq!(byte_to_bitarray(0b10010110), [true, false, false, true,
                                              false, true, true, false]);
}

#[test]
fn test_inversion() {
    for i in 0..256 {
        assert_eq!(i, bitarray_to_byte(byte_to_bitarray(i)));
    }
}
