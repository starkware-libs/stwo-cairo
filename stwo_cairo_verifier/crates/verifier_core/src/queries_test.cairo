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
            27196798, 28219483, 31894395, 72339167, 84432190, 109768375, 121482459, 135576248,
            141090327, 201124350, 226379824, 260773061, 267049178, 268388067, 271098949, 273043562,
            301884400, 319890920, 321792972, 334255406, 371402319, 406937719, 429056060, 436566787,
            462118705, 483657299, 511906017, 537131802, 550405635, 583915783, 634103812, 672019264,
            672088077, 686023726, 699359705, 706792812, 716176853, 720042540, 720871847, 782181960,
            786686392, 814197382, 841103847, 861407418, 861532150, 883526902, 907851892, 932280259,
            947873278, 957985107, 959555498, 970676526, 1012433978, 1088231635, 1172424897,
            1199095091, 1202524442, 1224313939, 1226301460, 1226933055, 1229079669, 1231446078,
            1255114545, 1311143116, 1355290654, 1386773311, 1395934751, 1406647032, 1409810875,
            1411866072, 1412505373, 1414877100, 1441827242, 1462495512, 1463563881, 1511781166,
            1515371129, 1544939519, 1565160877, 1572576997, 1590876528, 1642864596, 1653061951,
            1670958848, 1675802651, 1736364769, 1767378532, 1811209441, 1824225236, 1828264833,
            1828844968, 1831815099, 1920680993, 1947510621, 1950497831, 1963571540, 2003706137,
            2066089701, 2092416675, 2114908411,
        ]
            .span(),
        log_domain_size: 31,
    };
    assert_eq!(expected_result, result);
}
