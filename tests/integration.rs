use packed_integers::*;

#[test]
fn get_has_span() {
    let v1 = vec![507, 508, 509, 510, 511];

    let mut v2 = PackedIntegers::<U9>::new();
    for x in &v1 {
        v2.push(*x);
    }

    assert_eq!(v2.len(), v1.len());
    assert_eq!(v2.get(0).unwrap(), v1[0]);
    assert_eq!(v2.get(1).unwrap(), v1[1]);
    assert_eq!(v2.get(2).unwrap(), v1[2]);
    assert_eq!(v2.get(3).unwrap(), v1[3]);
    assert_eq!(v2.get(4).unwrap(), v1[4]);
}

#[test]
fn get_no_span() {
    let v1 = vec![251, 252, 253, 254, 255];

    let mut v2 = PackedIntegers::<U8>::new();
    for x in &v1 {
        v2.push(*x);
    }

    assert_eq!(v2.len(), v1.len());
    assert_eq!(v2.get(0).unwrap(), v1[0]);
    assert_eq!(v2.get(1).unwrap(), v1[1]);
    assert_eq!(v2.get(2).unwrap(), v1[2]);
    assert_eq!(v2.get(3).unwrap(), v1[3]);
    assert_eq!(v2.get(4).unwrap(), v1[4]);
}

#[test]
#[should_panic]
fn get_unchecked() {
    let v1 = vec![251, 252, 253, 254, 255];

    let mut v2 = PackedIntegers::<U8>::new();
    for x in &v1 {
        v2.push(*x);
    }

    assert_eq!(v2.get_unchecked(0), v1[0]);
    assert_eq!(v2.get_unchecked(1), v1[1]);
    assert_eq!(v2.get_unchecked(2), v1[2]);
    assert_eq!(v2.get_unchecked(3), v1[3]);
    assert_eq!(v2.get_unchecked(4), v1[4]);

    // UB if index >= len.
    assert_eq!(v2.get_unchecked(5), 0); // Fine.
    assert_eq!(v2.get_unchecked(6), 0); // Fine.
    assert_eq!(v2.get_unchecked(7), 0); // Fine.
    v2.get_unchecked(8); // Panics.
}

#[test]
fn into_iter_move() {
    let v1 = vec![251, 252, 253, 254, 255];

    let mut v2 = PackedIntegers::<U8>::new();
    for x in &v1 {
        v2.push(*x);
    }

    let mut iter = v2.into_iter();
    assert_eq!(iter.next().unwrap(), v1[0]);
    assert_eq!(iter.next().unwrap(), v1[1]);
    assert_eq!(iter.next().unwrap(), v1[2]);
    assert_eq!(iter.next().unwrap(), v1[3]);
    assert_eq!(iter.next().unwrap(), v1[4]);
    assert_eq!(iter.next(), None);

    // Moved. Compile error:
    // v2.push(250);
}

#[test]
fn into_iter_ref() {
    let v1 = vec![507, 508, 509, 510, 511];

    let mut v2 = PackedIntegers::<U9>::new();
    for x in &v1 {
        v2.push(*x);
    }

    let mut iter = (&v2).into_iter();
    assert_eq!(iter.next().unwrap(), v1[0]);
    assert_eq!(iter.next().unwrap(), v1[1]);
    assert_eq!(iter.next().unwrap(), v1[2]);
    assert_eq!(iter.next().unwrap(), v1[3]);
    assert_eq!(iter.next().unwrap(), v1[4]);
    assert_eq!(iter.next(), None);

    // Ok:
    // v2.push(506);
}

#[test]
fn iter() {
    let v1 = vec![507, 508, 509, 510, 511];

    let mut v2 = PackedIntegers::<U9>::new();
    for x in &v1 {
        v2.push(*x);
    }

    let mut iter = v2.iter();
    assert_eq!(iter.next().unwrap(), v1[0]);
    assert_eq!(iter.next().unwrap(), v1[1]);
    assert_eq!(iter.next().unwrap(), v1[2]);
    assert_eq!(iter.next().unwrap(), v1[3]);
    assert_eq!(iter.next().unwrap(), v1[4]);
    assert_eq!(iter.next(), None);

    // Ok:
    // v2.push(506);
}

#[test]
fn push_eq_max() {
    let mut v = PackedIntegers::<U10>::new();
    v.push(1023);
}

#[test]
#[should_panic]
fn push_gt_max() {
    let mut v = PackedIntegers::<U10>::new();
    v.push(1024);
}

#[test]
fn set() {
    let v1 = vec![251, 252, 253, 254, 255];

    let mut v2 = PackedIntegers::<U8>::new();
    for x in &v1 {
        v2.push(*x);
    }
    v2.set(0, 100);
    v2.set(2, 150);
    v2.set(4, 200);

    assert_eq!(v2.get(0).unwrap(), 100);
    assert_eq!(v2.get(1).unwrap(), v1[1]);
    assert_eq!(v2.get(2).unwrap(), 150);
    assert_eq!(v2.get(3).unwrap(), v1[3]);
    assert_eq!(v2.get(4).unwrap(), 200);
}

#[test]
#[should_panic]
fn set_oob() {
    let mut v = PackedIntegers::<U8>::new();
    v.push(100);
    v.set(1, 200);
}

#[test]
#[should_panic]
fn set_unchecked() {
    let mut v = PackedIntegers::<U8>::new();
    v.push(100);
    v.set_unchecked(1, 101); // Fine.
    v.set_unchecked(2, 102); // Fine.
    v.set_unchecked(3, 103); // Fine.
    v.set_unchecked(4, 104); // Panics.
}
