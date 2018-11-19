use phylo::{Phylo, Leaf, Node};
use ::util::{binom2, factorial};

impl<T> Phylo<T> where T: Eq {
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
        match self {
            Leaf(..)     => 0,
            Node(ref ts) => {
                self.is_symmetric() as u32 +
                    ts.iter().map(|t| t.count_symmetries()).sum::<u32>()
            }
        }
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

    pub fn sackin_index(&self) -> u32 {
        let (sackin, _) = self.fold::<(u32, u32), _>((0, 1), |iter| {
            let (sum_sackin, kappa) =
                iter.fold((0, 0), |(s,k), (s1,k1)| (s+s1, k+k1));

            (sum_sackin + kappa, kappa)
        });

        sackin
    }

    pub fn binary_colless_index(&self) -> u32 {
        let (colless, _) = self.binary_fold::<(i32, i32), _>((0, 1), |(cil, kappal), (cir, kappar)| {
            ((kappal-kappar).abs() + cil + cir, kappal + kappar)
        });

        colless as u32
    }

    pub fn cophenetic_index(&self) -> u32 {
        let (coph, _) = self.fold::<(u32, u32), _>((0, 1), |iter| {
            let (s, kappa) = iter.fold((0, 0), |(s,k), (c1,k1)| (s+c1, k+k1));
            let coph = binom2(s) + kappa;
            (coph, kappa)
        });

        coph
    }
}
