use babyjubjub_rs::*;
use ff::{Field, PrimeField, SqrtField};
use num_bigint::BigInt;
use rand_core::{OsRng, RngCore};

/*
Montgomery form
y^2 = x^3 + 168698x^2 + x

Twisted Edwards Form (standard)
ax^2 + y^2 = 1 + dx^2y^2

Parameters: a = 168700, d = 168696

168700 * x^2 + y^2 = 1 + 168696 * x^2 * y^2 =>
y^2 - 168696 * x^2 * y^2 = 1 - 168700 * x^2 =>
(1 - 168696 * x^2) * y^2 = 1 - 168700 * x^2 =>
y^2 = (1 - 168700 * x^2) * inv(1 - 168696 * x^2)
y = sqrt((1 - 168700 * x^2) * inv(1 - 168696 * x^2))
*/

pub fn from_x(x: Fr) -> Option<Point> {
    let mut x2 = x;
    x2.square();
    let x2 = x2;

    let mut a = Fr::from_repr(168700.into()).unwrap();
    a.mul_assign(&x2);
    let ax2 = a;
    let mut one = Fr::one();
    one.sub_assign(&ax2);
    let mut rhs_1 = one;

    let mut d = Fr::from_repr(168696.into()).unwrap();
    d.mul_assign(&x2);
    let dx2 = d;
    let mut one = Fr::one();
    one.sub_assign(&dx2);
    let rhs_2 = one.inverse();

    rhs_2?;

    let rhs_2 = rhs_2.unwrap();

    rhs_1.mul_assign(&rhs_2);

    rhs_1.sqrt().map(|y| Point { x, y })
}

pub fn is_on_curve(p: &Point) -> bool {
    let mut x = p.x;
    x.square();
    let x2 = x;

    let mut y = p.y;
    y.square();
    let y2 = y;

    let mut a = Fr::from_repr(168700.into()).unwrap();
    a.mul_assign(&x2);
    let mut ax2 = a;
    ax2.add_assign(&y2);
    let lhs = ax2;

    let mut d = Fr::from_repr(168696.into()).unwrap();
    d.mul_assign(&x2);
    d.mul_assign(&y2);
    let dx2y2 = d;
    let mut one = Fr::one();
    one.add_assign(&dx2y2);
    let rhs = one;

    lhs == rhs
}

pub fn random_point() -> Point {
    loop {
        let mut x = Fr::from_repr(OsRng.next_u64().into()).unwrap();
        x.mul_assign(&Fr::from_repr(OsRng.next_u64().into()).unwrap());
        x.mul_assign(&Fr::from_repr(OsRng.next_u64().into()).unwrap());
        x.mul_assign(&Fr::from_repr(OsRng.next_u64().into()).unwrap());
        if let Some(p) = from_x(x) {
            assert!(is_on_curve(&p));
            return p;
        }
    }
}

pub fn is_identity(p: &Point) -> bool {
    p.x == Fr::zero() && p.y == Fr::one()
}

pub fn identity() -> Point {
    Point {
        x: Fr::zero(),
        y: Fr::one(),
    }
}

pub fn is_on_small_subgroup(p: &Point) -> bool {
    let p2 = p.mul_scalar(&BigInt::from(2_i32));
    let p4 = p.mul_scalar(&BigInt::from(4_i32));
    let p8 = p.mul_scalar(&BigInt::from(8_i32));
    is_identity(&p2) || is_identity(&p4) || is_identity(&p8)
}

pub fn compute_r(p: &Point) -> Option<Point> {
    assert!(is_on_curve(p));

    let l = BigInt::parse_bytes(
        b"2736030358979909402780800718157159386076813972158567259200215660948447373041",
        10,
    )
    .unwrap();

    let q = p.mul_scalar(&l);

    match !is_identity(&q) {
        true => Some(q),
        false => None,
    }
}

pub fn random_small() -> Point {
    loop {
        let p = random_point();
        let r = compute_r(&p);
        if let Some(s) = r {
            return s;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ff::to_hex;
    use poseidon_rs::Poseidon;
    use std::collections::BTreeSet;

    #[test]
    fn test_find_point_m8() {
        let points = find_points_m8();
        println!("Found {} points.", points.len());
        println!("{points:?}");
    }

    fn find_points_m8() -> BTreeSet<Point> {
        let mut points = BTreeSet::<Point>::default();
        loop {
            let p = random_point();
            let r = compute_r(&p);
            if let Some(s) = r {
                if is_on_small_subgroup(&s) {
                    for i in 1..=8 {
                        let si = s.mul_scalar(&BigInt::from(i as i32));
                        points.insert(si);
                    }
                    break;
                }
            }
        }
        points
    }

    /*
    Point = Point { x: Fr(0x0999e47227c47e8829b0d14b251feed7582d0f5357b304b06d4014ae6ab39f1e), y: Fr(0x0aabb7211172d31463c1127d4ba6942cb791c56d8234b5a0abfd1e81afd0d677) }

    Point = Point { x: Fr(0x29da35670cdb640a2b6e763c0012aba803937ddf46736ee7b8e2b15b9d51c107), y: Fr(0x0000000000000000000000000000000000000000000000000000000000000000) }

    Point = Point { x: Fr(0x0999e47227c47e8829b0d14b251feed7582d0f5357b304b06d4014ae6ab39f1e), y: Fr(0x25b89751cfbecd15548f333935dac43070a222daf784baf097e4d712402f298a) }

    Point = Point { x: Fr(0x0000000000000000000000000000000000000000000000000000000000000000), y: Fr(0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000000) }

    Point = Point { x: Fr(0x26ca6a00b96d21a18e9f746b5c616985d006d8f522066be0d6a1e0e5854c60e3), y: Fr(0x25b89751cfbecd15548f333935dac43070a222daf784baf097e4d712402f298a) }

    Point = Point { x: Fr(0x068a190bd4563c1f8ce1cf7a816eacb524a06a69334601a98aff443852ae3efa), y: Fr(0x0000000000000000000000000000000000000000000000000000000000000000) }

    Point = Point { x: Fr(0x26ca6a00b96d21a18e9f746b5c616985d006d8f522066be0d6a1e0e5854c60e3), y: Fr(0x0aabb7211172d31463c1127d4ba6942cb791c56d8234b5a0abfd1e81afd0d677) }

    Point = Point { x: Fr(0x0000000000000000000000000000000000000000000000000000000000000000), y: Fr(0x0000000000000000000000000000000000000000000000000000000000000001) }
    */
    #[test]
    fn find_subgroup8() {
        let x = Fr::from_str(
            "4342719913949491028786768530115087822524712248835451589697801404893164183326",
        )
        .unwrap();
        let y = Fr::from_str(
            "4826523245007015323400664741523384119579596407052839571721035538011798951543",
        )
        .unwrap();
        let p = Point { x, y };
        assert!(is_on_curve(&p));
        println!("Point = {p:?}\n");
        for i in 2..=8 {
            let q = p.mul_scalar(&BigInt::from(i as i32));
            assert!(is_on_curve(&q));
            println!("Point = {q:?}\n");
        }
    }

    #[test]
    fn find_large_subgroup_point() {
        let l = BigInt::parse_bytes(
            b"2736030358979909402780800718157159386076813972158567259200215660948447373041",
            10,
        )
        .unwrap();
        loop {
            let p = random_point();
            let pp = p.mul_scalar(&l);
            let pp2 = p.mul_scalar(&(l.clone() * 2));
            let pp4 = p.mul_scalar(&(l.clone() * 4));
            let pp8 = p.mul_scalar(&(l.clone() * 8));
            if is_identity(&pp) {
                println!("Found point pP = {p:?}");
                break;
            }
            if is_identity(&pp2) {
                println!("Found point pP2 = {p:?}");
                break;
            }
            if is_identity(&pp4) {
                println!("Found point pP4 = {p:?}");
                break;
            }
            if is_identity(&pp8) {
                println!("Found point pP8 = {p:?}");
                break;
            }
        }
    }

    #[test]
    fn test_identity() {
        let id = Point {
            x: Fr::zero(),
            y: Fr::one(),
        };
        let mut id2 = id.mul_scalar(&BigInt::from(2 as i32));
        for _i in 0..1000000 {
            id2 = id.mul_scalar(&BigInt::from(2 as i32));
        }
        assert!(id.x == id2.x);
        assert!(id.y == id2.y);
    }

    #[test]
    fn test_ctf() {
        let x = Fr::from_str(
            "4342719913949491028786768530115087822524712248835451589697801404893164183326",
        )
        .unwrap();
        let y = Fr::from_str(
            "4826523245007015323400664741523384119579596407052839571721035538011798951543",
        )
        .unwrap();
        let a = Point { x, y };

        // keccak256(abi.encode("Baby it's me, ADDRESS))
        let msg_str =
            "1598911868658412840590204800029132395788172267694994825569831863639583317607";
        let msg = msg_str.parse::<BigInt>().unwrap();
        let msg_fr: Fr = Fr::from_str(&msg.to_string()).unwrap();

        let points_8 = find_points_m8();
        for r in points_8 {
            // We compute the RHS just to chec
            let k_fr = Poseidon::new()
                .hash(vec![r.x, r.y, a.x, a.y, msg_fr])
                .unwrap();
            let k = BigInt::parse_bytes(to_hex(&k_fr).as_bytes(), 16).unwrap();

            let k_a = a.mul_scalar(&k);
            let rhs = r.projective().add(&k_a.projective()).affine();

            let s = BigInt::from(0);
            let sig = Signature { r_b8: r.clone(), s };

            if verify(a.clone(), sig.clone(), msg.clone()) {
                assert!(is_identity(&rhs));
                println!("R: {:?}", r);
            }
        }
    }
}
