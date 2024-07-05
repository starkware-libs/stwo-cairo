use stwo_cairo_verifier::fields::m31::M31Trait;
use super::domain::{Coset, CosetImpl, LineDomain, CircleDomain, CircleDomainImpl, LineDomainImpl, dummy_line_domain};
use stwo_cairo_verifier::fields::qm31::{QM31, qm31, qm31_from_m31};
use stwo_cairo_verifier::fields::m31::M31;
use super::utils::{bit_reverse_index, pow};
use super::verifier::{FOLD_STEP, CIRCLE_TO_LINE_FOLD_STEP};

#[derive(Drop)]
pub struct LineEvaluation {
    pub values: Array<QM31>,
    pub domain: LineDomain
}

#[derive(Debug, Drop, Clone, PartialEq, Eq)]
pub struct CircleEvaluation {
    pub values: Array<QM31>,
    pub domain: CircleDomain,
}

#[derive(Drop)]
pub struct SparseLineEvaluation {
    pub subline_evals: Array<LineEvaluation>,
}

#[generate_trait]
pub impl SparseLineEvaluationImpl of SparseLineEvaluationTrait {
    fn fold(self: @SparseLineEvaluation, alpha: QM31) -> Array<QM31> {
        let mut i = 0;
        let mut res: Array<QM31> = array![];
        while i < self.subline_evals.len() {
            let line_evaluation = fold_line(self.subline_evals[i], alpha);
            res.append(*line_evaluation.values.at(0));
            i += 1;
        };
        return res;
    }
}

fn fold_line(eval: @LineEvaluation, alpha: QM31) -> LineEvaluation {
    let domain = eval.domain;
    let mut values: Array<QM31> = array![];
    let mut i = 0;
    // TODO: is this loop needed? The calling method only uses the first element of `value`
    while i < eval.values.len() / 2 {
        let x = domain.at(bit_reverse_index(i * pow(2, FOLD_STEP), domain.log_size()));
        let f_x = eval.values[2 * i];
        let f_neg_x = eval.values[2 * i + 1];
        let (f0, f1) = ibutterfly(*f_x, *f_neg_x, x.inverse());
        values.append(f0 + alpha * f1);
        i += 1;
    };

    LineEvaluationImpl::new(domain.double(), values)
}

pub fn ibutterfly(v0: QM31, v1: QM31, itwid: M31) -> (QM31, QM31) {
    (v0 + v1, (v0 - v1) * qm31_from_m31(itwid))
}


#[derive(Drop, Clone, Debug, PartialEq)]
pub struct SparseCircleEvaluation {
    pub subcircle_evals: Array<CircleEvaluation>
}

#[generate_trait]
pub impl LineEvaluationImpl of LineEvaluationTrait {
    fn new(domain: LineDomain, values: Array<QM31>) -> LineEvaluation {
        assert_eq!(values.len(), domain.size());
        LineEvaluation { values: values, domain: domain }
    }
}

#[generate_trait]
pub impl SparseCircleEvaluationImpl of SparseCircleEvaluationImplTrait {
    fn accumulate(
        self: @SparseCircleEvaluation, rhs: @SparseCircleEvaluation, alpha: QM31
    ) -> SparseCircleEvaluation {
        assert_eq!(self.subcircle_evals.len(), rhs.subcircle_evals.len());
        let mut subcircle_evals = array![];
        let mut i = 0;
        while i < self.subcircle_evals.len() {
            let lhs = self.subcircle_evals[i];
            let rhs = rhs.subcircle_evals[i];
            let mut values = array![];
            assert_eq!(lhs.values.len(), rhs.values.len());
            let mut j = 0;
            while j < lhs.values.len() {
                values.append(*lhs.values[j] * alpha + *rhs.values[j]);
                j += 1;
            };
            subcircle_evals.append(CircleEvaluation { domain: *lhs.domain, values });
            i += 1;
        };

        SparseCircleEvaluation { subcircle_evals }
    }

    fn fold(self: @SparseCircleEvaluation, alpha: QM31) -> Array<QM31> {
        let mut i = 0;
        let mut res: Array<QM31> = array![];
        while i < self.subcircle_evals.len() {
            let circle_evaluation = fold_circle_into_line(self.subcircle_evals[i], alpha);
            res.append(*circle_evaluation.values.at(0));
            i += 1;
        };
        return res;
    }
}

fn fold_circle_into_line(eval: @CircleEvaluation, alpha: QM31) -> LineEvaluation {
    let domain = eval.domain;
    let mut values = array![];
    let mut i = 0;
    while i < eval.values.len() / 2 {
        let p = domain.at(bit_reverse_index(i * pow(2, CIRCLE_TO_LINE_FOLD_STEP), domain.log_size()));
        let f_p = eval.values[2 * i];
        let f_neg_p = eval.values[2 * i + 1];
        let (f0, f1) = ibutterfly(*f_p, *f_neg_p, p.y.inverse());
        values.append(f0 + alpha * f1);
        i += 1;
    };
    LineEvaluation{values, domain: LineDomainImpl::new(*domain.half_coset)}
}

#[test]
fn test_accumulate_1() {
    let alpha = qm31(984186742, 463356387, 533637878, 1417871498);
    let lhs = SparseCircleEvaluation {
        subcircle_evals: array![
            CircleEvaluation {
                values: array![qm31(28672, 0, 0, 0), qm31(36864, 0, 0, 0)],
                domain: CircleDomain { half_coset: CosetImpl::new(16777216, 0) }
            },
            CircleEvaluation {
                values: array![qm31(28672, 0, 0, 0), qm31(36864, 0, 0, 0)],
                domain: CircleDomain { half_coset: CosetImpl::new(2030043136, 0) }
            },
            CircleEvaluation {
                values: array![qm31(2147454975, 0, 0, 0), qm31(2147446783, 0, 0, 0)],
                domain: CircleDomain { half_coset: CosetImpl::new(2097152000, 0) }
            },
        ]
    };
    let rhs = lhs.clone();
    let result = lhs.accumulate(@rhs, alpha);

    let expected_result = SparseCircleEvaluation {
        subcircle_evals: array![
            CircleEvaluation {
                values: array![
                    qm31(667173716, 1020487722, 1791736788, 1346152946),
                    qm31(1471361534, 84922130, 1076528072, 810417939)
                ],
                domain: CircleDomain { half_coset: CosetImpl::new(16777216, 0) }
            },
            CircleEvaluation {
                values: array![
                    qm31(667173716, 1020487722, 1791736788, 1346152946),
                    qm31(1471361534, 84922130, 1076528072, 810417939)
                ],
                domain: CircleDomain { half_coset: CosetImpl::new(2030043136, 0) }
            },
            CircleEvaluation {
                values: array![
                    qm31(1480309931, 1126995925, 355746859, 801330701),
                    qm31(676122113, 2062561517, 1070955575, 1337065708)
                ],
                domain: CircleDomain { half_coset: CosetImpl::new(2097152000, 0) }
            },
        ]
    };

    assert_eq!(expected_result, result);
}

#[test]
fn test_fold_line_1() {
    let domain = LineDomainImpl::new(CosetImpl::new(67108864, 1));
    let values = array![
        qm31(440443058, 1252794525, 1129773609, 1309365757),
        qm31(974589897, 1592795796, 649052897, 863407657)
    ];
    let sparse_line_evaluation = SparseLineEvaluation {
        subline_evals: array![LineEvaluation { values, domain }]
    };
    let alpha = qm31(1047716961, 506143067, 1065078409, 990356067);
    let result = sparse_line_evaluation.fold(alpha);
    let expected_result = array![qm31(515899232, 1030391528, 1006544539, 11142505)];
    assert_eq!(expected_result, result);
}

#[test]
fn test_fold_line_2() {
    let domain = LineDomainImpl::new(CosetImpl::new(553648128, 1));
    let values = array![
        qm31(730692421, 1363821003, 2146256633, 106012305),
        qm31(1387266930, 149259209, 1148988082, 1930518101)
    ];
    let sparse_line_evaluation = SparseLineEvaluation {
        subline_evals: array![LineEvaluation { values, domain }]
    };
    let alpha = qm31(2084521793, 1326105747, 548635876, 1532708504);
    let result = sparse_line_evaluation.fold(alpha);
    let expected_result = array![qm31(1379727866, 1083096056, 1409020369, 1977903500)];
    assert_eq!(expected_result, result);
}


#[test]
fn test_fold_circle_into_line_1() {
    let domain = CircleDomain{ half_coset: CosetImpl::new(553648128, 0)};
    let values = array![
        qm31(807167738, 0, 0, 0),
        qm31(1359821401, 0, 0, 0)
    ];
    let sparse_circle_evaluation = SparseCircleEvaluation {
        subcircle_evals: array![CircleEvaluation { values, domain }]
    };
    let alpha = qm31(260773061, 362745443, 1347591543, 1084609991);
    let result = sparse_circle_evaluation.fold(alpha);
    let expected_result = array![qm31(730692421, 1363821003, 2146256633, 106012305)];
    assert_eq!(expected_result, result);
}
