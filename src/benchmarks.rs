/*
Copyright 2016 Avi Weinstock

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use prefetch;
use test::{Bencher, black_box};

const CACHE_LINE_SIZE: usize = 64;

#[bench]
fn vec_sum_no_prefetch(b: &mut Bencher) {
    let data = (0..2*CACHE_LINE_SIZE).map(|i| (i % 256) as u8).collect::<Vec<u8>>();

    b.iter(|| {
        let mut sum = 0u64;
        for x in data.iter() {
            sum += *x as u64;
        }
        black_box(sum);
    });
}

#[bench]
fn vec_sum_with_prefetch(b: &mut Bencher) {
    let data = (0..2*CACHE_LINE_SIZE).map(|i| (i % 256) as u8).collect::<Vec<u8>>();

    b.iter(|| {
        let mut sum = 0u64;
        prefetch::prefetch::<prefetch::Read, prefetch::Medium, prefetch::Data, _>(data.as_ptr());
        for x in data.iter() {
            sum += *x as u64;
        }
        black_box(sum);
    });
}
