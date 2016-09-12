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

use llvmint;
use std::mem;

// http://llvm.org/docs/LangRef.html#llvm-prefetch-intrinsic
/*
pub enum ReadWrite { Read, Write }
pub enum Locality { None, Low, Medium, High }
pub enum CacheType { Instruction, Data }
*/
// These are promoted type-level versions of the above enums, since "The rw, locality and cache type arguments must be constant integers."
pub unsafe trait ReadWrite { const VALUE: i32; }
pub struct Read; unsafe impl ReadWrite for Read { const VALUE: i32 = 0; }
pub struct Write; unsafe impl ReadWrite for Write { const VALUE: i32 = 1; }

pub unsafe trait Locality { const VALUE: i32; }
pub struct None; unsafe impl Locality for None { const VALUE: i32 = 0; }
pub struct Low; unsafe impl Locality for Low { const VALUE: i32 = 1; }
pub struct Medium; unsafe impl Locality for Medium { const VALUE: i32 = 2; }
pub struct High; unsafe impl Locality for High { const VALUE: i32 = 3; }

pub unsafe trait CacheType { const VALUE: i32; }
pub struct Instruction; unsafe impl CacheType for Instruction { const VALUE: i32 = 0; }
pub struct Data; unsafe impl CacheType for Data { const VALUE: i32 = 1; }

#[inline(always)]
pub fn prefetch<RW: ReadWrite, Loc: Locality, Cache: CacheType, T>(x: *const T) {
    // This should be exposable as safe, since "Prefetches have no effect on the behavior of the program but can change its performance characteristics."
    unsafe { llvmint::prefetch(mem::transmute::<*const T, *mut i8>(x), RW::VALUE, Loc::VALUE, Cache::VALUE); }
}
