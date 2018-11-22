use lazy_static::lazy_static;

use phylo::{Phylo, Leaf, Node};
use shape::Shape;
use util::{binom2, factorial};


lazy_static! {
    static ref TRIPLET: Shape = make_shape!{ (*, *, *) };
}


lazy_static! {
    static ref QUARTETS: [Shape; 5] = [
        make_shape!{ (*, (*, (*, *))) },
        make_shape!{ (*, *, (*, *))   },
        make_shape!{ (*, (*, *, *))   },
        make_shape!{ ((*, *), (*, *)) },
        make_shape!{ (*, *, *, *)     }
    ];
}


impl<T: Eq> Phylo<T> {
    pub fn is_symmetric(&self) -> bool {
        match self {
            Leaf(..)     => true,
            Node(ref ts) => {
                let t0 = &ts[0];
                ts.iter().skip(1).all(|t| t.isomorphic(t0))
            }
        }
    }

    pub fn count_symmetries(&self) -> u32 {
        self.fold(0u32, |t, it| t.is_symmetric() as u32 + it.sum::<u32>())
    }

    pub fn count_automorphisms(&self) -> u32 {
        match self {
            Leaf(..) => 1,

            Node(ref ts) => {
                let mut aut: u32 = 1;
                let mut cur_sym_class_rep: Option<&Self> = None;
                let mut cur_sym_class_aut: u32 = 1;
                let mut cur_sym_class_len: u32 = 1;

                fn compute_aut_factor(class_aut: u32, class_len: u32) -> u32 {
                    class_aut.pow(class_len) * factorial(class_len)
                }

                for ti in ts.iter() {
                    if cur_sym_class_rep.is_none() || !cur_sym_class_rep.unwrap().isomorphic(ti) {
                        aut *= compute_aut_factor(cur_sym_class_aut, cur_sym_class_len);

                        cur_sym_class_rep = Some(ti);
                        cur_sym_class_aut = ti.count_automorphisms();
                        cur_sym_class_len = 1;
                    } else {
                        cur_sym_class_len += 1;
                    }
                }

                aut *= compute_aut_factor(cur_sym_class_aut, cur_sym_class_len);
                aut
            }
        }
    }
}

impl<T> Phylo<T> {
    pub fn sackin_index(&self) -> u32 {
        let (sackin, _) = self.fold((0u32, 1u32), |_, iter| {
            let (sum_sackin, kappa) =
                iter.fold((0, 0), |(s,k), (s1,k1)| (s+s1, k+k1));

            (sum_sackin + kappa, kappa)
        });

        sackin
    }

    pub fn binary_colless_index(&self) -> u32 {
        let (colless, _) = self.binary_fold((0i32, 1i32), |_, (cil, kappal), (cir, kappar)| {
            ((kappal-kappar).abs() + cil + cir, kappal + kappar)
        });

        colless as u32
    }

    pub fn cophenetic_index(&self) -> u32 {
        let (coph, kappa) = self.fold((0u32, 1u32), |_, iter| {
            let (s, kappa) = iter.fold((0, 0), |(s,k), (c1,k1)| (s+c1, k+k1));
            let coph = binom2(kappa) + s;
            (coph, kappa)
        });

        match self {
            Leaf(..) => 0,
            Node(..) => coph - binom2(kappa)
        }
    }

    pub fn binary_quartet_index(&self) -> u32 {
        let (qi, _) = self.binary_fold((0u32, 1u32), |_, (qi1, k1), (qi2, k2)| {
            let kappa = k1+k2;

            if kappa < 4 {
                (0, kappa)

            } else {
                let s0 = qi1 + qi2;
                let s3 = binom2(k1) * binom2(k2);

                (s0+s3, kappa)
            }
        });

        qi
    }

    pub fn quartet_index(&self, quartet_values: Option<&[u32; 5]>) -> u32 {
        let vs = quartet_values.unwrap_or(&[0,1,2,3,4]);

        let r = self.fold(QI::leaf(), |t, it| QI::from_rec(t, &it.collect::<Vec<_>>(), vs));

        return r.quartets;
    }
}


#[derive(Clone, Copy)]
struct QI {
    pub quartets: u32,
    triplets: u32,
    kappa: u32
}

impl QI {
    fn leaf() -> QI {
        QI {
            quartets: 0,
            triplets: 0,
            kappa: 1
        }
    }

    fn from_rec<T>(t: &Phylo<T>, qi_rec: &[QI], quartet_values: &[u32; 5]) -> QI {
        let k = qi_rec.len();

        let kappa = qi_rec.iter().map(|s| s.kappa).sum();

        let triplets = QI::compute_triplets(t, k, kappa, &qi_rec);

        let quartets = QI::compute_quartets(t, k, kappa, &qi_rec, quartet_values);

        QI {
            quartets: quartets,
            triplets: triplets,
            kappa: kappa
        }
    }

    fn compute_triplets<T>(t: &Phylo<T>, k: usize, kappa: u32, qi_rec: &[QI]) -> u32 {
        if kappa < 3 {
            0
        } else if t.isomorphic(&TRIPLET) {
            1
        } else {
            let t_s0: u32 = qi_rec.iter().map(|s| s.triplets).sum();

            let t_s1: u32 = map_indices!(i1 in 0..k, i2 in i1+1..k, i3 in i2+1..k => {
                qi_rec[i1].kappa * qi_rec[i2].kappa * qi_rec[i3].kappa
            }).sum();

            t_s0 + t_s1
        }
    }

    fn compute_quartets<T>(t: &Phylo<T>, k: usize, kappa: u32, qi_rec: &[QI], quartet_values: &[u32; 5]) -> u32 {
        match kappa {
            n if n < 4 => 0,
            4 => *QUARTETS.iter().zip(quartet_values).find(|(q, _)| t.isomorphic(q)).unwrap().1,
            _ => {
                let s0: u32 = qi_rec.iter().map(|s| s.quartets).sum();

                let s1: u32 = map_indices!(i1 in 0..k, i2 in i1+1..k, i3 in i2+1..k =>
                        binom2(qi_rec[i1].kappa) * qi_rec[i2].kappa * qi_rec[i3].kappa +
                        binom2(qi_rec[i2].kappa) * qi_rec[i1].kappa * qi_rec[i3].kappa +
                        binom2(qi_rec[i3].kappa) * qi_rec[i1].kappa * qi_rec[i2].kappa)
                    .sum();

                let s2: u32 = map_indices!(i1 in 0..k, i2 in i1+1..k =>
                        qi_rec[i1].kappa * qi_rec[i2].triplets +
                        qi_rec[i2].kappa * qi_rec[i1].triplets)
                    .sum();

                let s3: u32 = map_indices!(i1 in 0..k, i2 in i1+1..k =>
                        binom2(qi_rec[i1].kappa) * binom2(qi_rec[i2].kappa))
                    .sum();

                let s4: u32 = map_indices!(i1 in 0..k, i2 in i1+1..k, i3 in i2+1..k, i4 in i3+1..k =>
                        qi_rec[i1].kappa * qi_rec[i2].kappa * qi_rec[i3].kappa * qi_rec[i4].kappa)
                    .sum();

                s0 + quartet_values[1]*s1 + quartet_values[2]*s2 + quartet_values[3]*s3 + quartet_values[4]*s4
            }
        }
    }
}
