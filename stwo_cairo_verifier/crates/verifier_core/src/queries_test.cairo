#[cfg(feature: "poseidon252_verifier")]
use crate::channel::Channel;
use crate::queries::{Queries, QueriesImpl};

#[test]
fn test_fold_1() {
    let queries = Queries { positions: array![5].span(), log_domain_size: 6 };
    let result = queries.fold(1);
    let expected_result = Queries { positions: array![2].span(), log_domain_size: 5 };
    assert_eq!(expected_result, result);
}

#[test]
fn test_fold_2() {
    let queries = Queries { positions: array![17, 27].span(), log_domain_size: 6 };
    let result = queries.fold(1);
    let expected_result = Queries { positions: array![8, 13].span(), log_domain_size: 5 };
    assert_eq!(expected_result, result);
}

#[test]
#[cfg(feature: "poseidon252_verifier")]
fn test_generate() {
    let mut channel: Channel = Default::default();
    let result = QueriesImpl::generate(ref channel, 31, 100);
    let expected_result = Queries {
        positions: array![
            24873733, 54328540, 74615773, 78287782, 92884661, 98154792, 113884840, 166868429,
            168084405, 169518133, 188972610, 196767209, 213303138, 234971292, 239858110, 245782230,
            247436717, 258896369, 268110652, 276076674, 307038185, 310683107, 312597656, 334848724,
            370794486, 373862842, 374346000, 398114987, 444520293, 455380250, 476293665, 477679824,
            490499517, 510160379, 554437782, 568741525, 595916192, 620854952, 633877287, 682730141,
            684703334, 707353181, 717446133, 780874705, 797400586, 803769610, 814490712, 845674151,
            852664427, 855977816, 858560903, 886766026, 905998684, 939946039, 957251823, 993817728,
            998591402, 1024723229, 1035539173, 1042259504, 1071056379, 1075240091, 1083206030,
            1101907759, 1124684923, 1137339500, 1159213132, 1165467420, 1187401801, 1203308652,
            1256416054, 1282268748, 1294658240, 1309478258, 1361484013, 1381076658, 1381728512,
            1410703696, 1439556735, 1443340158, 1490849297, 1659504511, 1660362351, 1726030373,
            1739449785, 1742076025, 1753357659, 1761062340, 1769011850, 1798314219, 1834862422,
            1863857357, 1911410005, 1946614234, 1967485781, 1976693127, 2027004096, 2064620790,
            2101459525, 2106044809,
        ]
            .span(),
        log_domain_size: 31,
    };
    assert_eq!(expected_result, result);
}
