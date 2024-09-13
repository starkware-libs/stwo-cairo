use core::option::OptionTrait;
use stwo_cairo_verifier::channel::ChannelTrait;
use stwo_cairo_verifier::utils::{pow, find};
use stwo_cairo_verifier::channel::Channel;
use stwo_cairo_verifier::queries::{SparseSubCircleDomain, SubCircleDomain};
use stwo_cairo_verifier::sort::MinimumToMaximumSortedIterator;


#[derive(Drop, Clone, Debug, PartialEq, Eq)]
pub struct Queries {
    pub positions: Array<usize>,
    pub log_domain_size: u32,
}

#[generate_trait]
pub impl QueriesImpl of QueriesImplTrait {
    fn generate(ref channel: Channel, log_domain_size: u32, n_queries: usize) -> Queries {
        let mut nonsorted_positions = array![];
        let max_query = pow(2, log_domain_size) - 1;
        let mut finished = false;
        loop {
            let random_bytes = channel.draw_random_bytes();
            let mut i = 0;
            while i + 3 < random_bytes.len() {
                let b0: u32 = (*random_bytes[i + 0]).into();
                let b1: u32 = (*random_bytes[i + 1]).into();
                let b2: u32 = (*random_bytes[i + 2]).into();
                let b3: u32 = (*random_bytes[i + 3]).into();
                nonsorted_positions
                    .append((b0 + 256 * b1 + 65536 * b2 + 16777216 * b3) & max_query);
                if nonsorted_positions.len() == n_queries {
                    finished = true;
                    break;
                }
                i += 4;
            };
            if finished {
                break;
            }
        };

        // Sort and deduplicate
        let mut positions = array![];
        let mut iterator = MinimumToMaximumSortedIterator::new();
        while let Option::Some((x, _)) = iterator.iterate(nonsorted_positions.span()) {
            positions.append(x);
        };

        Queries { positions, log_domain_size }
    }

    fn len(self: @Queries) -> usize {
        self.positions.len()
    }

    fn fold(self: @Queries, n_folds: u32) -> Queries {
        assert!(n_folds <= *self.log_domain_size);
        assert!(self.positions.len() > 0);
        let folding_factor = pow(2, n_folds);
        let mut new_last_position = *self.positions[0] / folding_factor;
        let mut new_positions = array![new_last_position];
        let mut i = 1;
        while i < self.positions.len() {
            let new_position = *self.positions[i] / folding_factor;
            if new_position != new_last_position {
                new_last_position = new_position;
                new_positions.append(new_last_position);
            }
            i += 1;
        };
        Queries { positions: new_positions, log_domain_size: *self.log_domain_size - n_folds }
    }

    fn opening_positions(self: @Queries, fri_step_size: u32) -> SparseSubCircleDomain {
        assert!(fri_step_size > 0);
        let mut domains = array![];
        let snap_positions = self.positions;
        let mut already_added = array![];
        let mut i = 0;
        while i < snap_positions.len() {
            let v = *snap_positions.at(i);
            if(!find(v, already_added.span())) {
                already_added.append(v);
                domains.append(SubCircleDomain {
                    coset_index: v / pow(2, fri_step_size),
                    log_size: fri_step_size
                });
            }
            
            i = i + 1;
        };

        SparseSubCircleDomain {
            domains: domains,
            large_domain_log_size: *self.log_domain_size,
        }
    }
}


#[test]
fn test_fold_1() {
    let queries = Queries { positions: array![5], log_domain_size: 6 };
    let result = queries.fold(1);
    let expected_result = Queries { positions: array![2], log_domain_size: 5 };
    assert_eq!(expected_result, result);
}

#[test]
fn test_fold_2() {
    let queries = Queries { positions: array![17, 27], log_domain_size: 6 };
    let result = queries.fold(1);
    let expected_result = Queries { positions: array![8, 13], log_domain_size: 5 };
    assert_eq!(expected_result, result);
}

#[test]
fn test_generate() {
    let mut channel = ChannelTrait::new(0x00);
    let result = QueriesImpl::generate(ref channel, 31, 100);
    let expected_result = Queries {
        positions: array![
            27196798,
            28219483,
            31894395,
            72339167,
            84432190,
            109768375,
            121482459,
            135576248,
            141090327,
            201124350,
            226379824,
            260773061,
            267049178,
            268388067,
            271098949,
            273043562,
            301884400,
            319890920,
            321792972,
            334255406,
            371402319,
            406937719,
            429056060,
            436566787,
            462118705,
            483657299,
            511906017,
            537131802,
            550405635,
            583915783,
            634103812,
            672019264,
            672088077,
            686023726,
            699359705,
            706792812,
            716176853,
            720042540,
            720871847,
            782181960,
            786686392,
            814197382,
            841103847,
            861407418,
            861532150,
            883526902,
            907851892,
            932280259,
            947873278,
            957985107,
            959555498,
            970676526,
            1012433978,
            1088231635,
            1172424897,
            1199095091,
            1202524442,
            1224313939,
            1226301460,
            1226933055,
            1229079669,
            1231446078,
            1255114545,
            1311143116,
            1355290654,
            1386773311,
            1395934751,
            1406647032,
            1409810875,
            1411866072,
            1412505373,
            1414877100,
            1441827242,
            1462495512,
            1463563881,
            1511781166,
            1515371129,
            1544939519,
            1565160877,
            1572576997,
            1590876528,
            1642864596,
            1653061951,
            1670958848,
            1675802651,
            1736364769,
            1767378532,
            1811209441,
            1824225236,
            1828264833,
            1828844968,
            1831815099,
            1920680993,
            1947510621,
            1950497831,
            1963571540,
            2003706137,
            2066089701,
            2092416675,
            2114908411
        ],
        log_domain_size: 31
    };
    assert_eq!(expected_result, result);
}
